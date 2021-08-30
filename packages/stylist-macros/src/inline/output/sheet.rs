use super::{ContextRecorder, IntoCowVecTokens, OutputScopeContent, Reify};
use proc_macro2::TokenStream;
use quote::quote;

pub struct OutputSheet {
    pub contents: Vec<OutputScopeContent>,
}

impl Reify for OutputSheet {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        let Self { contents } = self;
        let contents = contents.into_cow_vec_tokens(ctx);

        quote! {
            {
                use ::std::convert::{From, Into};
                use ::stylist::ast::Sheet;
                <Sheet as From<Sheet>>::from(#contents)
            }
        }
    }
}
