///only works if all enum variants without fields like this
/// #[derive(proc_macro_assistants::GenerateToUpperCamelCase)]
/// enum Operation {
///     One,
///     Two,
///     Three,
/// }
#[proc_macro_derive(GenerateToUpperCamelCase)]
pub fn generate_to_upper_camel_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    let proc_macro_name_camel_case = "GenerateToUpperCamelCase";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_camel_case} {}: {e}",
            proc_macro_helpers::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_camel_case_ident_stringified = format!("{proc_macro_name_camel_case} {ident}");
    let data_enum = if let syn::Data::Enum(data_enum) = &ast.data {
        data_enum
    } else {
        panic!("{proc_macro_name_camel_case_ident_stringified} does work only on structs!");
    };
    let variants_matching_values_token_stream = data_enum.variants.iter().map(|variant| match &variant.fields {
        syn::Fields::Unit => {
            let variant_ident = &variant.ident;
            let variant_ident_upper_camel_case_stringified = proc_macro_helpers::to_upper_camel_case::ToUpperCamelCase::to_upper_camel_case(&variant_ident.to_string());//todo rename all _camel_case to _upper_camel_case
            let variant_ident_upper_camel_case_quotes_token_stream = proc_macro_helpers::generate_quotes::generate_quotes_token_stream(
                &variant_ident_upper_camel_case_stringified,
                &proc_macro_name_camel_case_ident_stringified,
            );
            quote::quote! {Self::#variant_ident => #variant_ident_upper_camel_case_quotes_token_stream}
        },
        _ => panic!("{proc_macro_name_camel_case} supported only syn::Fields::Unit"),
    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    let gen = quote::quote! {
        impl #ident {
            fn to_camel_case(&self) -> &str {
                match self {
                    #(#variants_matching_values_token_stream),*
                }
            }
        }
    };
    // println!("{gen}");
    gen.into()
}