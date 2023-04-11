use super::*;

pub enum StateExpr<T>
where
    T: Codegen,
{
    Raw(T::Stmt),
    Block(StateBlock<T>),
    If(StateIf<T>),
    Match(StateMatch<T>),
    While(StateWhile<T>),
    For(StateForLoop<T>),
    Command(StateCommand<T>),
}

impl<T: Codegen> Parse for StateExpr<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let r = if input.peek(Token![if]) {
            StateExpr::If(input.parse()?)
        } else if input.peek(Token![match]) {
            StateExpr::Match(input.parse()?)
        } else if input.peek(Token![while]) {
            StateExpr::While(input.parse()?)
        } else if input.peek(Token![for]) {
            StateExpr::For(input.parse()?)
        } else if input.peek(Brace) {
            StateExpr::Block(input.parse()?)
        } else if input.peek(Token!(@)) {
            StateExpr::Command(input.parse()?)
        } else {
            StateExpr::Raw(input.parse()?)
        };
        Ok(r)
    }
}

macro_rules! impl_to_tokens {
    ($($Arm:ident)*) => {
        impl<T: Codegen> ToTokens for StateExpr<T> {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                match self {
                    $(StateExpr::$Arm(x) => x.to_tokens(tokens),)*
                }
            }
        }
    }
}

impl_to_tokens!(Raw Block If Match While For Command);

macro_rules! impl_from {
    ($($Arm:ident $Type:ident,)*) => {
        $(
            impl<T: Codegen> From<$Type<T>> for StateExpr<T> {
                fn from(e: $Type<T>) -> Self {
                    Self::$Arm(e)
                }
            }
        )*
    };
}

impl_from! {
    Block StateBlock,
    If StateIf,
    Match StateMatch,
    While StateWhile,
    For StateForLoop,
    Command StateCommand,
}

macro_rules! impl_block_visit {
    ($($Arm:ident)*) => {
        impl<T: Codegen> VisitUnit<T> for StateExpr<T> {
            fn visit_unit<F>(&self, f: &mut F)
            where
                F: FnMut(&StateExpr<T>)
            {
                match self {
                    $(Self::$Arm(x) => x.visit_unit(f),)*
                    x @ Self::Command(_) | x @ Self::Raw(_) => {
                        f(x)
                    }
                }
            }

            fn visit_unit_mut<F>(&mut self, f: &mut F)
            where
                F: FnMut(&mut StateExpr<T>)
            {
                match self {
                    $(Self::$Arm(x) => x.visit_unit_mut(f),)*
                    x @ Self::Command(_) | x @ Self::Raw(_) => {
                        f(x)
                    }
                }
            }
        }
    };
}

impl_block_visit!(Block If Match While For);