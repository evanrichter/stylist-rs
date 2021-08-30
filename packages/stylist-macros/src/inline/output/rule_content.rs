use super::{ContextRecorder, OutputAtRule, OutputQualifiedRule, Reify};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Error as ParseError;

pub enum OutputRuleContent {
    AtRule(OutputAtRule),
    Block(OutputQualifiedRule),
    Err(ParseError),
}

impl Reify for OutputRuleContent {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        match self {
            Self::AtRule(rule) => {
                let block_tokens = rule.into_token_stream(ctx);
                quote! { ::stylist::ast::RuleContent::Rule(::std::boxed::Box::new(#block_tokens)) }
            }
            Self::Block(block) => {
                let block_tokens = block.into_token_stream(ctx);
                quote! { ::stylist::ast::RuleContent::Block(#block_tokens) }
            }
            Self::Err(err) => err.into_token_stream(ctx),
        }
    }
}
