fn generate_function_name_snake_case_token_stream(
    proc_macro_name_upper_camel_case_stringified: &str,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let value = proc_macro_helpers::naming_conventions::ToSnakeCase::to_snake_case(&proc_macro_name_upper_camel_case_stringified);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
/*
only works if all enum variants without fields like this
#[derive(proc_macro_assistants::ToUpperCamelCase)]
enum Operation {
     One,
     Two,
     Three,
}
*/
#[proc_macro_derive(ToUpperCamelCase)]
pub fn to_upper_camel_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "ToUpperCamelCase";
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
            let variant_ident_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::ToUpperCamelCase::to_upper_camel_case(&variant_ident.to_string());
            let variant_ident_upper_camel_case_quotes_token_stream = proc_macro_helpers::generate_quotes::generate_quotes_token_stream(
                &variant_ident_upper_camel_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {Self::#variant_ident => #variant_ident_upper_camel_case_quotes_token_stream}
        },
        _ => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let function_name_snake_case_token_stream = generate_function_name_snake_case_token_stream(
        &proc_macro_name_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let gen = quote::quote! {
        impl #ident {
            fn #function_name_snake_case_token_stream(&self) -> &str {
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
 #[derive(proc_macro_assistants::ToSnakeCase)]
 enum Operation {
     One,
     Two,
     Three,
 }
*/
#[proc_macro_derive(ToSnakeCase)]
pub fn to_snake_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "ToSnakeCase";
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
            let variant_ident_snake_case_stringified = proc_macro_helpers::naming_conventions::ToSnakeCase::to_snake_case(&variant_ident.to_string());
            let variant_ident_snake_case_quotes_token_stream = proc_macro_helpers::generate_quotes::generate_quotes_token_stream(
                &variant_ident_snake_case_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            quote::quote! {Self::#variant_ident => #variant_ident_snake_case_quotes_token_stream}
        },
        _ => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let function_name_snake_case_token_stream = generate_function_name_snake_case_token_stream(
        &proc_macro_name_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let gen = quote::quote! {
        impl #ident {
            fn #function_name_snake_case_token_stream(&self) -> &str {
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{gen}");
    gen.into()
}

#[proc_macro_derive(ParametersUpperCamelCaseTokenStream)]
pub fn parameters_upper_camel_case_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    let proc_macro_name_upper_camel_case_stringified = "ParametersUpperCamelCaseTokenStream";
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
            let ident_parameters_upper_camel_case_token_stream = {
                let parameters_upper_camel_case_stringified = "Parameters";
                let value = format!(
                    "{}{parameters_upper_camel_case_stringified}",
                    proc_macro_helpers::naming_conventions::ToUpperCamelCase::to_upper_camel_case(&variant_ident.to_string())
                );
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_helpers::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote! {Self::#variant_ident => quote::quote! {#ident_parameters_upper_camel_case_token_stream}}
        },
        _ => panic!("{proc_macro_name_upper_camel_case_stringified} supported only syn::Fields::Unit"),
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let function_name_snake_case_token_stream = generate_function_name_snake_case_token_stream(
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