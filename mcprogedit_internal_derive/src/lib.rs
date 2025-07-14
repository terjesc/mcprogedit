use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields, GenericArgument, PathArguments, Type};

#[proc_macro_derive(BlockApi)]
pub fn block_api_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_block_api(&ast)
}

#[derive(Debug)]
struct ParsedEnum {
    ident: proc_macro2::Ident,
    variants: Vec<Variant>,
}

#[derive(Debug)]
struct Variant {
    ident: proc_macro2::Ident,
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
    Direct {
        field_name: proc_macro2::Ident,
        field_type: proc_macro2::Ident,
    },
    Optioned {
        field_name: proc_macro2::Ident,
        field_type: proc_macro2::Ident,
    },
}

#[derive(Debug)]
enum UnnamedEnumField {
    Direct(proc_macro2::Ident),
    Boxed(proc_macro2::Ident),
    Optioned(proc_macro2::Ident),
}

fn parse_enum_fields(data: &syn::DataEnum) -> Vec<Variant> {
    let mut variants = Vec::new();

    for variant in &data.variants {
        let variant_ident = &variant.ident;

        match &variant.fields {
            // E.g. "Foo,"
            Fields::Unit => variants.push(Variant {
                ident: variant_ident.clone(),
                variant_type: VariantType::Unit,
            }),

            // E.g. "Foo { bar: Baz },"
            Fields::Named(fields) => {
                let mut named_fields = Vec::new();

                for field in &fields.named {
                    let field_name = field.ident.as_ref().unwrap_or_else(|| {
                        panic!(
                            "No name found for a field in enum variant {}",
                            variant_ident,
                        )
                    });
                    if let Type::Path(type_path) = &field.ty {
                        let type_segments = &type_path.path.segments;
                        let type_segment = &type_segments[0];

                        // The type is either direct, or wrapped in an Option
                        if &type_segment.ident == "Option" {
                            if let PathArguments::AngleBracketed(argument) = &type_segment.arguments
                            {
                                if let GenericArgument::Type(Type::Path(path)) = &argument.args[0] {
                                    let type_ident = &path.path.segments[0].ident;
                                    named_fields.push(NamedEnumField::Optioned {
                                        field_name: field_name.clone(),
                                        field_type: type_ident.clone(),
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
                                field_name: field_name.clone(),
                                field_type: type_ident.clone(),
                            });
                        }
                    }
                }

                variants.push(Variant {
                    ident: variant_ident.clone(),
                    variant_type: VariantType::Named(named_fields),
                });
            }

            // E.g. "Foo(Bar)," or "Foo(Box<Bar>)," or "Foo(Option<Bar>),"
            Fields::Unnamed(fields) => {
                let mut unnamed_fields = Vec::new();

                for field in &fields.unnamed {
                    if let Type::Path(type_path) = &field.ty {
                        let type_segments = &type_path.path.segments;
                        let type_segment = &type_segments[0];

                        // The type is either direct, wrapped in an Option, or wrapped in a Box
                        if &type_segment.ident == "Option" {
                            if let PathArguments::AngleBracketed(argument) = &type_segment.arguments
                            {
                                if let GenericArgument::Type(Type::Path(path)) = &argument.args[0] {
                                    let type_ident = &path.path.segments[0].ident;
                                    unnamed_fields
                                        .push(UnnamedEnumField::Optioned(type_ident.clone()));
                                } else {
                                    panic!()
                                }
                            } else {
                                panic!()
                            }
                        } else if &type_segment.ident == "Box" {
                            if let PathArguments::AngleBracketed(argument) = &type_segment.arguments
                            {
                                if let GenericArgument::Type(Type::Path(path)) = &argument.args[0] {
                                    let type_ident = &path.path.segments[0].ident;
                                    unnamed_fields
                                        .push(UnnamedEnumField::Boxed(type_ident.clone()));
                                } else {
                                    panic!()
                                }
                            } else {
                                panic!()
                            }
                        } else {
                            let type_ident = &type_segment.ident;
                            unnamed_fields.push(UnnamedEnumField::Direct(type_ident.clone()));
                        }
                    }
                }

                variants.push(Variant {
                    ident: variant_ident.clone(),
                    variant_type: VariantType::Unnamed(unnamed_fields),
                });
            }
        }
    }
    variants
}

fn impl_block_api(ast: &syn::DeriveInput) -> TokenStream {
    let main_ident = &ast.ident;

    let mut generated = Vec::new();

    match &ast.data {
        Data::Enum(data) => {
            let parsed_enum = ParsedEnum {
                ident: main_ident.clone(),
                variants: parse_enum_fields(&data),
            };

            println!("\nParsed enum structure:\n{:#?}", parsed_enum);
            // TODO Call functions for each type of attribute.
            // TODO Those functions will have subcalls to each impl.
            generated.push(impl_colour(&parsed_enum));
        }
        _ => panic!(
            "BlockApi can only be derived on enums. {} is not an enum.",
            main_ident
        ),
    }

    let generated = quote! {
        #(#generated)*
    };

    println!("Generated code:\n{:#?}", &generated);
    generated.into()
}

// TODO this function could probably be generalized, with the following parameters:
// * list of attribute names
// * list of data types
// * Maybe whether or not to include Option, as well?
fn impl_colour(parsed_enum: &ParsedEnum) -> proc_macro2::TokenStream {
    let ident = &parsed_enum.ident;

    // For implementation of `fn Block::has_colour(&self) -> bool`
    let has_colour_arms: Vec<_> = parsed_enum
        .variants
        .iter()
        .map(
            |Variant {
                 ident,
                 variant_type,
             }| {
                match variant_type {
                    VariantType::Unit => quote! { Self::#ident => false, },
                    VariantType::Named(variants) => {
                        //let mut arm = quote! { Self::#ident { .. } => false, };
                        let mut arm = quote! {};
                        for variant in variants {
                            match variant {
                                NamedEnumField::Direct { field_name, .. }
                                | NamedEnumField::Optioned { field_name, .. } => {
                                    if field_name == "colour" {
                                        arm = quote! { Self::#ident { .. } => true, };
                                    }
                                }
                            }
                        }
                        arm
                    }
                    VariantType::Unnamed(variants) => {
                        //let mut arm = quote! { Self::#ident( .. ) => todo!(), };
                        let mut arm = quote! {};
                        for variant in variants {
                            match variant {
                                UnnamedEnumField::Direct(field_type)
                                | UnnamedEnumField::Boxed(field_type)
                                | UnnamedEnumField::Optioned(field_type) => {
                                    if field_type == "Colour" {
                                        arm = quote! { Self::#ident( .. ) => true, };
                                    }
                                    //TODO: If there is a Box'ed struct: Call that struct's has_colour()
                                    //TODO: If there is a single struct: Call that struct's has_colour()?
                                    //TODO: Maybe require some annotation or something for structs?
                                }
                            }
                        }
                        arm
                    }
                }
            },
        )
        .collect();

    let generated = quote! {
        impl #ident {
            fn has_colour(&self) -> bool {
                match self {
                    #(#has_colour_arms)*
                    _ => false,
                }
            }
        }
    };

    generated
}

