use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    braced,
    parse::{Nothing, Parse},
    punctuated::Punctuated,
    token::Brace,
    Expr, Pat, Result, Token,
};

use crate::expr::{
    state_block::stmts_to_tokens, Codegen, ConditionalApplicator, StateExpr, VisitUnit,
};

/*
    match expr1 {
        Pat1(x) if expr2 => Arm1(...),
        Pat2(y) => Arm2(Arm1(...)),
        Pat3(z) => Arm2(Arm2(...)),
    }
*/
pub struct StateMatch<T: Codegen> {
    match_token: Token![match],
    expr: Expr,
    brace_token: Brace,
    arms: Vec<Arm<T>>,
}

pub struct Arm<T: Codegen> {
    pat: Pat,
    guard: Option<(Token![if], Expr)>,
    fat_arrow_token: Token![=>],
    body: Box<StateExpr<T>>,
}

impl<T: Codegen> Parse for StateMatch<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let match_token = input.parse()?;
        let expr = Expr::parse_without_eager_brace(input)?;

        let match_body;
        let brace_token = braced!(match_body in input);
        let arms = Punctuated::<_, Nothing>::parse_terminated_with(&match_body, |input| {
            Ok(Arm {
                pat: Pat::parse_multi_with_leading_vert(input)?,
                guard: {
                    if input.peek(Token![if]) {
                        Some((input.parse()?, input.parse()?))
                    } else {
                        None
                    }
                },
                fat_arrow_token: input.parse()?,
                body: {
                    let body: Box<StateExpr<T>> = input.parse()?;
                    if matches!(&*body, StateExpr::Raw(_)) && T::MUST_IN_BLOCK {
                        return Err(input.error(
                            "expression must be in block, consider move it into braces: `{expression}`.",
                        ));
                    }
                    input.parse::<Option<Token![,]>>()?;
                    body
                },
            })
        })?
        .into_iter()
        .collect();

        Ok(StateMatch {
            match_token,
            expr,
            brace_token,
            arms,
        })
    }
}

impl<T: Codegen> ToTokens for StateMatch<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let StateMatch {
            match_token,
            expr,
            brace_token: _,
            arms,
        } = self;

        match_token.to_tokens(tokens);
        expr.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            let mut applicator = T::conditional_applicate(arms.len());
            for arm in arms.iter() {
                arm_to_tokens(tokens, arm, &mut applicator);
            }
        });
    }
}

// pat if guard => expr,
fn arm_to_tokens<T: Codegen>(tokens: &mut TokenStream, arm: &Arm<T>, ca: &mut T::Ca) {
    let Arm {
        pat,
        guard,
        fat_arrow_token,
        body,
    } = arm;

    pat.to_tokens(tokens);

    if let Some((if_token, expr)) = guard {
        if_token.to_tokens(tokens);
        expr.to_tokens(tokens);
    }

    fat_arrow_token.to_tokens(tokens);

    match &**body {
        StateExpr::Block(block) => ca.apply(tokens, stmts_to_tokens(&block.stmts)),
        other => ca.apply(tokens, other),
    }
    <Token![,]>::default().to_tokens(tokens);
}

impl<T: Codegen> VisitUnit<T> for StateMatch<T> {
    fn visit_unit<'a, F>(&'a self, depth: usize, f: &mut F) -> Result<()>
    where
        F: FnMut(&'a StateExpr<T>, usize) -> Result<()>,
        T: 'a,
    {
        for x in &self.arms {
            x.body.visit_unit(depth + 1, f)?;
        }
        Ok(())
    }

    fn visit_unit_mut<'a, F>(&'a mut self, depth: usize, f: &mut F) -> Result<()>
    where
        F: FnMut(&'a mut StateExpr<T>, usize) -> Result<()>,
        T: 'a,
    {
        for x in &mut self.arms {
            x.body.visit_unit_mut(depth + 1, f)?;
        }
        Ok(())
    }
}
