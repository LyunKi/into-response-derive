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
                let bytes = match serde_json::to_vec(&self) {
                    Ok(res) => res,
                    Err(err) => {
                        return axum::response::Response::builder()
                            .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
                            .header(
                                hyper::header::CONTENT_TYPE,
                                axum::http::HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                            )
                            .body(axum::body::boxed(axum::body::Full::from(err.to_string())))
                            .unwrap();
                    }
                };
                let mut res =
                    axum::response::Response::new(axum::body::boxed(axum::body::Full::from(bytes)));
                res.headers_mut().insert(
                    hyper::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                );
                res
            }
        }
    };
    gen.into()
}
