#[proc_macro_derive(GenerateToCamelCaseEnumUnnamedVariants)]//todo check on postgresql max length value of type
pub fn generate_to_camel_case_enum_unnamed_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {//todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_helpers::panic_location::panic_location();
    let proc_macro_name_camel_case = "GenerateToCamelCaseEnumUnnamedVariants";
    // let proc_macro_name_lower_case = proc_macro_helpers::to_lower_snake_case::ToLowerSnakeCase::to_lower_snake_case(&proc_macro_name_camel_case);
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| {
        panic!(
            "{proc_macro_name_camel_case} {}: {e}",
            proc_macro_helpers::global_variables::hardcode::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", ast.data);
    let ident = &ast.ident;
    // let ident_lower_case_stringified = proc_macro_helpers::to_lower_snake_case::ToLowerSnakeCase::to_lower_snake_case(&ident.to_string());
    let proc_macro_name_camel_case_ident_stringified = format!("{proc_macro_name_camel_case} {ident}");
    let data_enum = if let syn::Data::Enum(data_enum) = &ast.data {
        data_enum
    } else {
        panic!("{proc_macro_name_camel_case_ident_stringified} does work only on structs!");
    };
    quote::quote! {
        impl Operation {
            //todo maybe write simple proc macro for it?
            fn to_camel_case(&self) -> &str {
                match self {
                    Self::CreateMany => "CreateMany",
                    Self::CreateOne => "CreateOne",
                    Self::ReadMany => "ReadMany",
                    Self::ReadOne => "ReadOne",
                    Self::UpdateMany => "UpdateMany",
                    Self::UpdateOne => "UpdateOne",
                    Self::DeleteMany => "DeleteMany",
                    Self::DeleteOne => "DeleteOne",
                }
            }
        }
    }.into()
}