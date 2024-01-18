/*
only works if all enum variants without fields like this
#[derive(proc_macro_assistants::ToUpperCamelCaseString)]
enum Operation {
     One,
     Two,
     Three,
}
*/
#[proc_macro_derive(ToUpperCamelCaseString)]
pub fn to_upper_camel_case_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "ToUpperCamelCaseString";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_upper_camel_case_stringified} {}: {e}",
            proc_macro_helpers::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case_stringified} {ident}");
    let data_enum = if let syn::Data::Enum(data_enum) = &ast.data {
        data_enum
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let std_string_string_token_stream = proc_macro_helpers::std_string_string_token_stream();
    let variants_matching_values_token_stream = data_enum.variants.iter().map(|variant| match &variant.fields {
        syn::Fields::Unit => {
            let variant_ident = &variant.ident;
            let variant_ident_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::ToUpperCamelCaseString::to_upper_camel_case_string(&variant_ident.to_string());
            let variant_ident_upper_camel_case_quotes_token_stream = proc_macro_helpers::generate_quotes::generate_quotes_token_stream(
                &variant_ident_upper_camel_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {Self::#variant_ident => #std_string_string_token_stream::from(#variant_ident_upper_camel_case_quotes_token_stream)}
        },
        _ => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let function_name_snake_case_token_stream = proc_macro_helpers::generate_function_name_snake_case_token_stream(
        &proc_macro_name_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let trait_path_token_stream = proc_macro_helpers::trait_path_token_stream();
    let proc_macro_name_upper_camel_case_token_stream = proc_macro_helpers::generate_function_name_upper_camel_case_token_stream(
        &proc_macro_name_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let gen = quote::quote! {
        impl #trait_path_token_stream::#proc_macro_name_upper_camel_case_token_stream for #ident {
            fn #function_name_snake_case_token_stream(&self) -> #std_string_string_token_stream {//todo maybe write duplicate Trait with &str instead of std::string::String
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{gen}");
    gen.into()
}

/* 
only works if all enum variants without fields like this
 #[derive(proc_macro_assistants::ToSnakeCaseString)]
 enum Operation {
     One,
     Two,
     Three,
 }
*/
#[proc_macro_derive(ToSnakeCaseString)]
pub fn to_snake_case_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "ToSnakeCaseString";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_upper_camel_case_stringified} {}: {e}",
            proc_macro_helpers::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case_stringified} {ident}");
    let data_enum = if let syn::Data::Enum(data_enum) = &ast.data {
        data_enum
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let std_string_string_token_stream = proc_macro_helpers::std_string_string_token_stream();
    let variants_matching_values_token_stream = data_enum.variants.iter().map(|variant| match &variant.fields {
        syn::Fields::Unit => {
            let variant_ident = &variant.ident;
            let variant_ident_snake_case_stringified = proc_macro_helpers::naming_conventions::ToSnakeCaseString::to_snake_case_string(&variant_ident.to_string());
            let variant_ident_snake_case_quotes_token_stream = proc_macro_helpers::generate_quotes::generate_quotes_token_stream(
                &variant_ident_snake_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {Self::#variant_ident => #std_string_string_token_stream::from(#variant_ident_snake_case_quotes_token_stream)}
        },
        _ => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let function_name_snake_case_token_stream = proc_macro_helpers::generate_function_name_snake_case_token_stream(
        &proc_macro_name_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let trait_path_token_stream = proc_macro_helpers::trait_path_token_stream();
    let proc_macro_name_upper_camel_case_token_stream = proc_macro_helpers::generate_function_name_upper_camel_case_token_stream(
        &proc_macro_name_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let gen = quote::quote! {
        impl #trait_path_token_stream::#proc_macro_name_upper_camel_case_token_stream for #ident {
            fn #function_name_snake_case_token_stream(&self) -> #std_string_string_token_stream {
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{gen}");
    gen.into()
}
/* 
only works if all enum variants without fields like this
 #[derive(proc_macro_assistants::ToScreamingSnakeCaseString)]
 enum Operation {
     One,
     Two,
     Three,
 }
*/
#[proc_macro_derive(ToScreamingSnakeCaseString)]
pub fn to_screaming_snake_case_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "ToScreamingSnakeCaseString";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_upper_camel_case_stringified} {}: {e}",
            proc_macro_helpers::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case_stringified} {ident}");
    let data_enum = if let syn::Data::Enum(data_enum) = &ast.data {
        data_enum
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let std_string_string_token_stream = proc_macro_helpers::std_string_string_token_stream();
    let variants_matching_values_token_stream = data_enum.variants.iter().map(|variant| match &variant.fields {
        syn::Fields::Unit => {
            let variant_ident = &variant.ident;
            let variant_ident_snake_case_stringified = proc_macro_helpers::naming_conventions::ToScreamingSnakeCaseString::to_screaming_snake_case_string(&variant_ident.to_string());
            let variant_ident_snake_case_quotes_token_stream = proc_macro_helpers::generate_quotes::generate_quotes_token_stream(
                &variant_ident_snake_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {Self::#variant_ident => #std_string_string_token_stream::from(#variant_ident_snake_case_quotes_token_stream)}
        },
        _ => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let function_name_snake_case_token_stream = proc_macro_helpers::generate_function_name_snake_case_token_stream(
        &proc_macro_name_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let trait_path_token_stream = proc_macro_helpers::trait_path_token_stream();
    let proc_macro_name_upper_camel_case_token_stream = proc_macro_helpers::generate_function_name_upper_camel_case_token_stream(
        &proc_macro_name_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let gen = quote::quote! {
        impl #trait_path_token_stream::#proc_macro_name_upper_camel_case_token_stream for #ident {
            fn #function_name_snake_case_token_stream(&self) -> #std_string_string_token_stream {
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{gen}");
    gen.into()
}

// #[proc_macro_derive(ParametersUpperCamelCaseTokenStream)]
// pub fn parameters_upper_camel_case_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     proc_macro_helpers::panic_location::panic_location();
//     let proc_macro_name_upper_camel_case_stringified = "ParametersUpperCamelCaseTokenStream";
//     let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
//         panic!(
//             "{proc_macro_name_upper_camel_case_stringified} {}: {e}",
//             proc_macro_helpers::global_variables::hardcode::AST_PARSE_FAILED
//         )
//     });
//     let ident = &ast.ident;
//     let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case_stringified} {ident}");
//     let data_enum = if let syn::Data::Enum(data_enum) = &ast.data {
//         data_enum
//     } else {
//         panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
//     };
//     let variants_matching_values_token_stream = data_enum.variants.iter().map(|variant| match &variant.fields {
//         syn::Fields::Unit => {
//             let variant_ident = &variant.ident;
//             let ident_parameters_upper_camel_case_token_stream = {
//                 let parameters_upper_camel_case_stringified = "Parameters";
//                 let value = format!(
//                     "{}{parameters_upper_camel_case_stringified}",
//                     proc_macro_helpers::naming_conventions::ToUpperCamelCase::to_upper_camel_case(&variant_ident.to_string())
//                 );
//                 value.parse::<proc_macro2::TokenStream>()
//                 .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//             };
//             quote::quote! {Self::#variant_ident => quote::quote! {#ident_parameters_upper_camel_case_token_stream}}
//         },
//         _ => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
//     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
//     let function_name_snake_case_token_stream = generate_function_name_snake_case_token_stream(
//         &proc_macro_name_upper_camel_case_stringified,
//         &proc_macro_name_upper_camel_case_ident_stringified,
//     );
//     let gen = quote::quote! {
//         impl #ident {
//             fn #function_name_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//                 match self {
//                     #(#variants_matching_values_token_stream),*
//                 }
//             }
//         }
//     };
//     // println!("{gen}");
//     gen.into()
// }

#[proc_macro_derive(PayloadUpperCamelCaseTokenStream)]
pub fn payload_upper_camel_case_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "PayloadUpperCamelCaseTokenStream";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_upper_camel_case_stringified} {}: {e}",
            proc_macro_helpers::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case_stringified} {ident}");
    let data_enum = if let syn::Data::Enum(data_enum) = &ast.data {
        data_enum
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let variants_matching_values_token_stream = data_enum.variants.iter().map(|variant| match &variant.fields {
        syn::Fields::Unit => {
            let variant_ident = &variant.ident;
            let ident_payload_upper_camel_case_token_stream = {
                let payload_upper_camel_case_stringified = "Payload";
                let value = format!(
                    "{}{payload_upper_camel_case_stringified}",
                    proc_macro_helpers::naming_conventions::ToUpperCamelCaseString::to_upper_camel_case_string(&variant_ident.to_string())
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote! {Self::#variant_ident => quote::quote! {#ident_payload_upper_camel_case_token_stream}}
        },
        _ => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let function_name_snake_case_token_stream = proc_macro_helpers::generate_function_name_snake_case_token_stream(
        &proc_macro_name_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let gen = quote::quote! {
        impl #ident {
            fn #function_name_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{gen}");
    gen.into()
}

//
// payload_with_serialize_deserialize_upper_camel_case_token_stream
#[proc_macro_derive(PayloadWithSerializeDeserializeUpperCamelCaseTokenStream)]
pub fn payload_with_serialize_deserialize_upper_camel_case_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "PayloadWithSerializeDeserializeUpperCamelCaseTokenStream";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_upper_camel_case_stringified} {}: {e}",
            proc_macro_helpers::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case_stringified} {ident}");
    let data_enum = if let syn::Data::Enum(data_enum) = &ast.data {
        data_enum
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let variants_matching_values_token_stream = data_enum.variants.iter().map(|variant| match &variant.fields {
        syn::Fields::Unit => {
            let variant_ident = &variant.ident;
            let ident_payload_with_serialize_deserialize_upper_camel_case_token_stream = {
                let payload_with_serialize_deserialize_upper_camel_case_stringified = "PayloadWithSerializeDeserialize";
                let value = format!(
                    "{}{payload_with_serialize_deserialize_upper_camel_case_stringified}",
                    proc_macro_helpers::naming_conventions::ToUpperCamelCaseString::to_upper_camel_case_string(&variant_ident.to_string())
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote! {Self::#variant_ident => quote::quote! {#ident_payload_with_serialize_deserialize_upper_camel_case_token_stream}}
        },
        _ => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let function_name_snake_case_token_stream = proc_macro_helpers::generate_function_name_snake_case_token_stream(
        &proc_macro_name_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let gen = quote::quote! {
        impl #ident {
            fn #function_name_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{gen}");
    gen.into()
}
//