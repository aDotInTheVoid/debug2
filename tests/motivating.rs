use debug2::pprint;

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

impl debug2::Debug2 for UnOp {
    fn fmt2(&self, f: &mut debug2::Formatter2<'_>) -> std::fmt::Result {
        match self {
            UnOp::Minus => f.debug_tuple("Minus").finish(),
            UnOp::Sqrt => f.debug_tuple("Sqrt").finish(),
        }
    }
}
impl debug2::Debug2 for BinOp {
    fn fmt2(&self, f: &mut debug2::Formatter2<'_>) -> std::fmt::Result {
        match self {
            BinOp::Div => f.debug_tuple("Div").finish(),
            BinOp::Mul => f.debug_tuple("Mul").finish(),
            BinOp::Minus => f.debug_tuple("Minus").finish(),
            BinOp::Pow => f.debug_tuple("Pow").finish(),
            BinOp::PlusMinus => f.debug_tuple("PlusMinus").finish(),
        }
    }
}
impl debug2::Debug2 for Instr {
    fn fmt2(&self, f: &mut debug2::Formatter2<'_>) -> std::fmt::Result {
        match self {
            Instr::Push(v) => f.debug_tuple("Push").field(v).finish(),
            Instr::Load(v) => f.debug_tuple("Load").field(v).finish(),
            Instr::BinOp(v) => f.debug_tuple("BinOp").field(v).finish(),
            Instr::UnOp(v) => f.debug_tuple("UnOp").field(v).finish(),
        }
    }
}

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

    assert_snapshot!(format!("{:?}", instrs));
    assert_snapshot!(format!("{:#?}", instrs));
    assert_snapshot!(pprint(instrs));
}
