use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(IntoResponse)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_into_response_macro(&ast)
}

fn impl_into_response_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics axum::response::IntoResponse for #name #ty_generics #where_clause{
            fn into_response(self) -> axum::response::Response {
                match serde_json::to_vec(&self) {
                    Ok(res) => (
                        [(
                            axum::http::header::CONTENT_TYPE,
                            axum::http::HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                        )],
                        res,
                    )
                        .into_response(),
                    Err(err) => (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        [(
                            axum::http::header::CONTENT_TYPE,
                            axum::http::HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                        )],
                        err.to_string(),
                    )
                        .into_response(),
                }
            }
        }
    };
    gen.into()
}
