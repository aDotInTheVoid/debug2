use debug2::pprint;
use insta::assert_snapshot;

macro_rules! check {
    ($e:expr) => {
        assert_snapshot!(pprint($e))
    };
}

#[derive(debug2::Debug)]
struct X {
    a: i32,
    b: i32,
}

#[derive(debug2::Debug)]
struct Wrapper<T>(T);

#[test]
fn simple() {
    let x = X { a: 1, b: 5 };

    check!(x);
}
