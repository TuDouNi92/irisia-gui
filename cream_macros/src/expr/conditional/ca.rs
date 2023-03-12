use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{token::Paren, TypePath};

use crate::expr::ConditionalApplicator;

pub struct DefaultConditionalApplicator {
    count: usize,
    skipped: usize,
    path: TypePath,
}

impl DefaultConditionalApplicator {
    pub fn new(count: usize, branch_path: TypePath) -> Self {
        DefaultConditionalApplicator {
            count,
            skipped: 0,
            path: branch_path,
        }
    }
}

impl ConditionalApplicator for DefaultConditionalApplicator {
    fn apply<F>(&mut self, tokens: &mut TokenStream, f: F)
    where
        F: FnOnce(&mut TokenStream),
    {
        if self.skipped == self.count {
            panic!("inner error: branches requested more than count");
        }

        let path = &self.path;

        let stream = if self.skipped == self.count - 1 {
            let mut stream = TokenStream::new();
            f(&mut stream);
            stream
        } else {
            let mut stream = quote!(#path::Arm1);
            Paren::default().surround(&mut stream, f);
            stream
        };

        add_arm2(tokens, path, self.skipped, &stream);
        self.skipped += 1;
    }
}

fn add_arm2(tokens: &mut TokenStream, path: &TypePath, depth: usize, expr: &TokenStream) {
    if depth == 0 {
        expr.to_tokens(tokens);
        return;
    }

    quote!(#path::Arm2).to_tokens(tokens);

    Paren::default().surround(tokens, |tokens| {
        add_arm2(tokens, path, depth - 1, expr);
    });
}