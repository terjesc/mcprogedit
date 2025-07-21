use syn::{Fields, GenericArgument, PathArguments, Type};

#[derive(Debug)]
pub(crate) struct ParsedEnum {
    pub ident: proc_macro2::Ident,
    pub variants: Vec<Variant>,
}

#[derive(Debug)]
pub(crate) struct Variant {
    pub ident: proc_macro2::Ident,
    pub variant_type: VariantType,
}

#[derive(Debug)]
pub(crate) enum VariantType {
    Unit,
    Named(Vec<NamedEnumField>),
    Unnamed(Vec<UnnamedEnumField>),
}

#[derive(Debug)]
pub(crate) enum NamedEnumField {
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
pub(crate) enum UnnamedEnumField {
    Direct(proc_macro2::Ident),
    Boxed(proc_macro2::Ident),
    Optioned(proc_macro2::Ident),
}

pub(crate) fn parse_enum_fields(data: &syn::DataEnum) -> Vec<Variant> {
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

