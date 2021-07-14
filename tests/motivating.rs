use debug2::{pprint, Debug};

use insta::assert_snapshot;

macro_rules! check {
    ($e:expr) => {
        assert_snapshot!(pprint($e))
    };
}

#[test]
fn complex() {
    let complex_structure = vec![
        vec![Some(1), Some(2), Some(3), None],
        vec![Some(2), None],
        vec![Some(4), Some(7)],
        vec![Some(1), Some(2), Some(3), None],
        vec![Some(2), None],
        vec![Some(4), Some(7)],
        vec![Some(1), Some(2), Some(3), None],
        vec![Some(2), None],
        vec![Some(4), Some(7)],
    ];

    check!(complex_structure);
}

#[derive(Debug)]
enum Instr {
    Push(i32),
    Load(&'static str),
    BinOp(BinOp),
    UnOp(UnOp),
}
#[derive(Debug)]
enum BinOp {
    Div,
    Mul,
    Minus,
    Pow,
    PlusMinus,
}
#[derive(Debug)]
enum UnOp {
    Minus,
    Sqrt,
}

// TODO: Derive these
// https://github.com/rust-lang/rust/blob/master/compiler/rustc_builtin_macros/src/deriving/debug.rs
// https://github.com/panicbit/custom_debug/blob/master/custom_debug_derive/src/lib.rs

#[test]
fn quadratic_form() {
    let instrs = vec![
        Instr::Load("b"),
        Instr::UnOp(UnOp::Minus),
        Instr::Load("b"),
        Instr::Push(2),
        Instr::BinOp(BinOp::Pow),
        Instr::Load("a"),
        Instr::Load("c"),
        Instr::Push(4),
        Instr::BinOp(BinOp::Mul),
        Instr::BinOp(BinOp::Mul),
        Instr::BinOp(BinOp::Minus),
        Instr::UnOp(UnOp::Sqrt),
        Instr::BinOp(BinOp::PlusMinus),
        Instr::Load("a"),
        Instr::Push(2),
        Instr::BinOp(BinOp::Div),
    ];

    assert_snapshot!(pprint(instrs));
}
