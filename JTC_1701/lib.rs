#![expect(incomplete_features)]
#![feature(
    generic_const_exprs,
    const_trait_impl,
    const_ops,
    explicit_tail_calls,
)]
#![allow(non_snake_case)]

extern crate balanced_ternary as ternary;

mod instr;
mod machine;
