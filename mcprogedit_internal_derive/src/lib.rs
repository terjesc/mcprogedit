use proc_macro::TokenStream;
use quote::quote;
use syn::{ Data, Fields, GenericArgument, PathArguments, Type };

#[proc_macro_derive(BlockApi)]
pub fn block_api_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_block_api(&ast)
}

#[derive(Debug)]
struct ParsedEnum {
    ident: String,
    variants: Vec<Variant>,
}

#[derive(Debug)]
struct Variant {
    ident: String,
    variant_type: VariantType,
}

#[derive(Debug)]
enum VariantType {
    Unit,
    Named(Vec<NamedEnumField>),
    Unnamed(Vec<UnnamedEnumField>),
}

#[derive(Debug)]
enum NamedEnumField {
    Direct { field_name: String, field_type: String },
    Optioned { field_name: String, field_type: String },
}

#[derive(Debug)]
enum UnnamedEnumField {
    Direct(String),
    Boxed(String),
    Optioned(String),
}

fn parse_enum_fields(data: &syn::DataEnum) -> Vec<Variant> {
    let mut variants = Vec::new();

    for variant in &data.variants {
        let variant_ident = &variant.ident;

        match &variant.fields {
            // E.g. "Foo,"
            Fields::Unit => variants.push(Variant {
                ident: variant_ident.to_string(),
                variant_type: VariantType::Unit,
            }),

            // E.g. "Foo { bar: Baz },"
            Fields::Named(fields) => {
                let mut named_fields = Vec::new();

                for field in &fields.named {
                    let field_name = &field.ident.as_ref().unwrap_or_else(|| panic!(
                        "No name found for a field in enum variant {}",
                        variant_ident,
                    ));
                    if let Type::Path(type_path) = &field.ty {
                        let type_segments = &type_path.path.segments;
                        let type_segment = &type_segments[0];

                        // The type is either direct, or wrapped in an Option
                        if &type_segment.ident == "Option" {
                            if let PathArguments::AngleBracketed(argument) = &type_segment.arguments {
                                if let GenericArgument::Type(Type::Path(path)) = &argument.args[0] {
                                    let type_ident = &path.path.segments[0].ident;
                                    named_fields.push(NamedEnumField::Optioned {
                                        field_name: field_name.to_string(),
                                        field_type: type_ident.to_string(),
                                    });
                                } else {
                                    panic!()
                                }
                            } else {
                                panic!()
                            }
                        } else {
                            let type_ident = &type_segment.ident;
                            named_fields.push(NamedEnumField::Direct {
                                field_name: field_name.to_string(),
                                field_type: type_ident.to_string(),
                            });
                        }
                    }
                }
                variants.push(Variant {
                    ident: variant_ident.to_string(),
                    variant_type: VariantType::Named(named_fields),
                });
            }

            // E.g. "Foo(Bar)," or "Foo(Box<Bar>),"
            // Fields::Unnamed(fields) => {}
            // TODO parse variants with unnamed fields, into Direct, Optioned and Boxed
            _ => println!("\nUnimplemented field:\n{:?}", &variant.fields),
        }
    }

    variants
}

fn impl_block_api(ast: &syn::DeriveInput) -> TokenStream {
    let main_ident = &ast.ident;

    match &ast.data {
        Data::Enum(data) => {
            let parsed_enum = ParsedEnum {
                ident: main_ident.to_string(),
                variants: parse_enum_fields(&data),
            };

            println!("\nParsed enum structure:\n{:#?}", parsed_enum);
            // TODO Call functions for each type of attribute.
            // TODO Those functions will have subcalls to each impl.
        }
        _ => panic!("BlockApi can only be derived on enums. {} is not an enum.", main_ident),
    }

    // TODO This is one way to generate an implementation, but it should be outsourced to separate
    //      functions
    let mut match_lines = Vec::new();
    match_lines.push(quote! { _ => false, });
    let generated = quote! {
        impl #main_ident {
            fn has_colour(&self) -> bool {
                match self {
                    #(#match_lines),*
                }
            }
        }
    };
    //println!("Generated code:\n{:#?}", &generated);
    generated.into()
}

