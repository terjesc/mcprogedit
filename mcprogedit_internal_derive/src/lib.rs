use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

mod parsed_enum;

use crate::parsed_enum::*;

#[proc_macro_derive(BlockApi)]
pub fn block_api_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_block_api(&ast)
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

