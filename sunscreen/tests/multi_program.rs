use sunscreen::{
    types::{bfv::Signed, Cipher},
    *,
};

#[test]
fn compiling_multiple_programs_yields_same_params() {
    #[fhe_program(scheme = "bfv")]
    fn add(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
        a + b
    }

    #[fhe_program(scheme = "bfv")]
    fn mul(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
        a * b
    }

    let app = Compiler::new()
        .fhe_program(add)
        .fhe_program(mul)
        .compile()
        .unwrap();

    assert_eq!(*app.params(), app.get_program(add).unwrap().metadata.params);
    assert_eq!(*app.params(), app.get_program(mul).unwrap().metadata.params);
}

#[test]
fn can_reference_program_strongly_or_stringly() {
    #[fhe_program(scheme = "bfv")]
    fn add(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
        a + b
    }

    #[fhe_program(scheme = "bfv")]
    fn mul(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
        a * b
    }

    let app = Compiler::new()
        .fhe_program(add)
        .fhe_program(mul)
        .compile()
        .unwrap();

    assert_eq!(mul.name(), "mul");
    assert_eq!(add.name(), "add");

    assert_eq!(app.get_program(mul).is_some(), true);
    assert_eq!(app.get_program("mul").is_some(), true);
    assert_eq!(app.get_program(add).is_some(), true);
    assert_eq!(app.get_program("add").is_some(), true);
}

#[test]
fn get_programs_iterates_every_program() {
    #[fhe_program(scheme = "bfv")]
    fn add(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
        a + b
    }

    #[fhe_program(scheme = "bfv")]
    fn mul(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
        a * b
    }

    let app = Compiler::new()
        .fhe_program(add)
        .fhe_program(mul)
        .compile()
        .unwrap();

    assert_eq!(app.get_programs().count(), 2);
    assert_eq!(app.get_programs().any(|(k, _)| k == "mul"), true);
    assert_eq!(app.get_programs().any(|(k, _)| k == "add"), true);
}

#[test]
fn cant_compile_multiple_programs_with_same_name() {
    mod foo {
        use super::*;

        #[fhe_program(scheme = "bfv")]
        pub fn add(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
            a + b
        }
    }

    mod bar {
        use super::*;

        #[fhe_program(scheme = "bfv")]
        pub fn add(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
            a + b
        }
    }

    let result = Compiler::new()
        .fhe_program(foo::add)
        .fhe_program(bar::add)
        .compile();

    match result {
        Err(Error::NameCollision) => {}
        _ => {
            panic!("Expected Error::NameCollision")
        }
    };
}
