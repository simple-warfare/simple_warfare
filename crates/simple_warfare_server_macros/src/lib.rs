use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, FieldsNamed, Ident, LitStr, parse_macro_input};

enum GenerateInput {
    NamedFieldStruct(FieldsNamed),
    TupleStruct(String, Vec<Attribute>),
}

/// Derives the `TryFromAndIntoJs` trait, with the `#[boa()]` attribute.
///
/// # Panics
///
/// It will panic if the user tries to derive the `TryFromJs` trait in an `enum` or a tuple struct.
#[proc_macro_derive(TryFromAndIntoJs, attributes(boa))]
pub fn derive_try_from_and_into_js(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Struct(data) = input.data else {
        panic!("you can only derive TryFromAndIntoJs for structs");
    };

    let type_name = input.ident;

    let (conv, props) = if let Fields::Named(fields) = data.fields {
        let generate_input = GenerateInput::NamedFieldStruct(fields);
        let conv = generate_conversion(&generate_input).unwrap_or_else(to_compile_errors);
        let props = generate_obj_properties(&generate_input)
            .map_err(|err| vec![err])
            .unwrap_or_else(to_compile_errors);
        (conv, props)
    } else if let Fields::Unnamed(fileds) = data.fields {
        if fileds.unnamed.len() != 1 {
            panic!(
                "you can only derive TryFromAndIntoJs for Unnamed-Filed Struct which has one menber"
            )
        }

        let generate_input = GenerateInput::TupleStruct(type_name.to_string(), input.attrs);
        let conv = generate_conversion(&generate_input).unwrap_or_else(to_compile_errors);
        let props = generate_obj_properties(&generate_input)
            .map_err(|err| vec![err])
            .unwrap_or_else(to_compile_errors);
        (conv, props)
    } else {
        panic!("you can only derive TryFromAndIntoJs for Unit Struct or Named-Filed Struct")
    };

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl ::boa_engine::value::TryFromJs for #type_name {
            fn try_from_js(value: &boa_engine::JsValue, context: &mut boa_engine::Context)
                -> boa_engine::JsResult<Self> {
                match value {
                    boa_engine::JsValue::Object(o) => {#conv},
                    _ => Err(boa_engine::JsError::from(
                        boa_engine::JsNativeError::typ()
                            .with_message("cannot convert value to a #type_name")
                    )),
                }
            }
        }

        impl ::boa_engine::value::TryIntoJs for #type_name {
            fn try_into_js(&self, context: &mut boa_engine::Context) -> boa_engine::JsResult<boa_engine::JsValue> {
                let obj = boa_engine::JsObject::default();
                #props
                boa_engine::JsResult::Ok(obj.into())
            }
        }
    };

    // Hand the output tokens back to the compiler
    expanded.into()
}

/// Generates the conversion field by field.
fn generate_conversion(input: &GenerateInput) -> Result<proc_macro2::TokenStream, Vec<syn::Error>> {
    use syn::spanned::Spanned;

    match input {
        GenerateInput::NamedFieldStruct(fields) => {
            let mut field_list = Vec::with_capacity(fields.named.len());
            let mut final_fields = Vec::with_capacity(fields.named.len());

            for field in fields.named.clone() {
                let span = field.span();

                let name = field.ident.ok_or_else(|| {
                    vec![syn::Error::new(
                        span,
                        "you can only derive `TryFromAndIntoJs` for named-field structs",
                    )]
                })?;

                field_list.push(name.clone());

                let mut from_js_with = None;
                let mut field_name = format!("{name}");
                if let Some(attr) = field
                    .attrs
                    .into_iter()
                    .find(|attr| attr.path().is_ident("boa"))
                {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("from_js_with") {
                            let value = meta.value()?;
                            from_js_with = Some(value.parse::<LitStr>()?);
                            Ok(())
                        } else if meta.path.is_ident("rename") {
                            let value = meta.value()?;
                            field_name = value.parse::<LitStr>()?.value();
                            Ok(())
                        } else if meta.path.is_ident("into_js_with") {
                            meta.value()?.parse::<LitStr>()?;
                            // ignore
                            Ok(())
                        } else {
                            Err(meta.error(
                                "invalid syntax in the `#[boa()]` attribute. \
                              Note that this attribute only accepts the following syntax: \
                            `#[boa(from_js_with = \"fully::qualified::path\")]`",
                            ))
                        }
                    })
                    .map_err(|err| vec![err])?;
                }

                let error_str = format!("cannot get property {name} of value");
                final_fields.push(quote! {
                    let #name = match props.get(&::boa_engine::js_string!(#field_name).into()) {
                        Some(pd) => pd.value().ok_or_else(|| ::boa_engine::JsError::from(
                                ::boa_engine::JsNativeError::typ().with_message(#error_str)
                            ))?.clone().try_js_into(context)?,
                        None => ::boa_engine::JsValue::undefined().try_js_into(context)?,
                    };
                });

                if let Some(method) = from_js_with {
                    let ident = Ident::new(&method.value(), method.span());
                    final_fields.push(quote! {
                        let #name = #ident(&#name, context)?;
                    });
                }
            }

            // TODO: this could possibly skip accessors. Consider using `JsObject::get` instead.
            Ok(quote! {
                let o = o.borrow();
                let props = o.properties();
                #(#final_fields)*
                Ok(Self {
                    #(#field_list),*
                })
            })
        }
        GenerateInput::TupleStruct(type_name, attrs) => {
            let mut from_js_with = None;
            let default_name =
                type_name.chars().nth(0).unwrap().to_lowercase().to_string() + &type_name[1..];
            let mut field_name = format!("{default_name}");
            let mut final_fields = Vec::with_capacity(2);

            if let Some(attr) = attrs.into_iter().find(|attr| attr.path().is_ident("boa")) {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("from_js_with") {
                        let value = meta.value()?;
                        from_js_with = Some(value.parse::<LitStr>()?);
                        Ok(())
                    } else if meta.path.is_ident("name") {
                        let value = meta.value()?;
                        field_name = value.parse::<LitStr>()?.value();
                        Ok(())
                    } else if meta.path.is_ident("into_js_with") {
                        meta.value()?.parse::<LitStr>()?;
                        // ignore
                        Ok(())
                    } else {
                        Err(meta.error(
                            "invalid syntax in the `#[boa()]` attribute. \
                              Note that this attribute only accepts the following syntax: \
                            `#[boa(from_js_with = \"fully::qualified::path\")]`",
                        ))
                    }
                })
                .map_err(|err| vec![err])?;
            }
            let error_str = format!("cannot get property {type_name} of value");

            final_fields.push(quote! {
                let value = match props.get(&::boa_engine::js_string!(#field_name).into()) {
                    Some(pd) => pd.value().ok_or_else(|| ::boa_engine::JsError::from(
                            ::boa_engine::JsNativeError::typ().with_message(#error_str)
                        ))?.clone().try_js_into(context)?,
                    None => ::boa_engine::JsValue::undefined().try_js_into(context)?,
                };
            });
            if let Some(method) = from_js_with {
                let ident = Ident::new(&method.value(), method.span());
                final_fields.push(quote! {
                    let value = #ident(&value, context)?;
                });
            }
            Ok(quote! {
                let o = o.borrow();
                let props = o.properties();
                #(#final_fields)*
                Ok(Self(value))
            })
        }
    }
}
/// Generates a list of compile errors.
#[allow(clippy::needless_pass_by_value)]
fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}

/// Generates property creation for object.
fn generate_obj_properties(input: &GenerateInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    use syn::spanned::Spanned;
    match input {
        GenerateInput::NamedFieldStruct(fields) => {
            let mut prop_ctors = Vec::with_capacity(fields.named.len());

            for field in fields.named.clone() {
                let span = field.span();
                let name = field.ident.ok_or_else(|| {
                    syn::Error::new(
                        span,
                        "you can only derive `TryIntoJs` for named-field structs",
                    )
                })?;

                let mut into_js_with = None;
                let mut prop_key = format!("{name}");
                let mut skip = false;

                for attr in field
                    .attrs
                    .into_iter()
                    .filter(|attr| attr.path().is_ident("boa"))
                {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("into_js_with") {
                            let value = meta.value()?;
                            into_js_with = Some(value.parse::<LitStr>()?);
                            Ok(())
                        } else if meta.path.is_ident("rename") {
                            let value = meta.value()?;
                            prop_key = value.parse::<LitStr>()?.value();
                            Ok(())
                        } else if meta.path.is_ident("skip") & meta.input.is_empty() {
                            skip = true;
                            Ok(())
                        } else if meta.path.is_ident("from_js_with") {
                            meta.value()?.parse::<LitStr>()?;
                            Ok(())
                        } else {
                            Err(meta.error(
                                "invalid syntax in the `#[boa()]` attribute. \
                              Note that this attribute only accepts the following syntax: \
                            \n* `#[boa(into_js_with = \"fully::qualified::path\")]`\
                            \n* `#[boa(rename = \"jsPropertyName\")]` \
                            \n* `#[boa(skip)]` \
                            ",
                            ))
                        }
                    })?;
                }

                if skip {
                    continue;
                }

                let value = if let Some(into_js_with) = into_js_with {
                    let into_js_with = Ident::new(&into_js_with.value(), into_js_with.span());
                    quote! { #into_js_with(&self.#name, context)? }
                } else {
                    quote! { boa_engine::value::TryIntoJs::try_into_js(&self.#name, context)? }
                };
                prop_ctors.push(quote! {
            obj.create_data_property_or_throw(boa_engine::js_string!(#prop_key), #value, context)?;
        });
            }

            Ok(quote! { #(#prop_ctors)* })
        }
        GenerateInput::TupleStruct(type_name, attrs) => {
            let mut into_js_with = None;
            let mut prop_key = format!("{type_name}");

            for attr in attrs.into_iter().filter(|attr| attr.path().is_ident("boa")) {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("into_js_with") {
                        let value = meta.value()?;
                        into_js_with = Some(value.parse::<LitStr>()?);
                        Ok(())
                    } else if meta.path.is_ident("name") {
                        let value = meta.value()?;
                        prop_key = value.parse::<LitStr>()?.value();
                        Ok(())
                    } else if meta.path.is_ident("skip") & meta.input.is_empty() {
                        Ok(())
                    } else if meta.path.is_ident("from_js_with") {
                        meta.value()?.parse::<LitStr>()?;
                        Ok(())
                    } else {
                        Err(meta.error(
                            "invalid syntax in the `#[boa()]` attribute. \
                              Note that this attribute only accepts the following syntax: \
                            \n* `#[boa(into_js_with = \"fully::qualified::path\")]`\
                            \n* `#[boa(rename = \"jsPropertyName\")]` \
                            \n* `#[boa(skip)]` \
                            ",
                        ))
                    }
                })?;
            }

            let value = if let Some(into_js_with) = into_js_with {
                let into_js_with = Ident::new(&into_js_with.value(), into_js_with.span());
                quote! { #into_js_with(&self.0, context)? }
            } else {
                quote! { boa_engine::value::TryIntoJs::try_into_js(&self.0, context)? }
            };
            Ok(quote! {
                obj.create_data_property_or_throw(boa_engine::js_string!(#prop_key), #value, context)?;
            })
        }
    }
}
