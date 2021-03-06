#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate derive_more;

use std::path::PathBuf;

// Here just to make sure that this doesn't conflict with
// the derives in some way
use std::fmt::Binary;

#[derive(Display, Octal, Binary)]
struct MyInt(i32);

#[derive(Display)]
#[display(fmt = "({}, {})", x, y)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Display)]
#[display(fmt = "{}", message)]
struct Error {
    message: &'static str,
    backtrace: (),
}

impl Error {
    fn new(message: &'static str) -> Self {
        Self {
            message,
            backtrace: (),
        }
    }
}

#[derive(Display)]
enum E {
    Uint(u32),
    #[display(fmt = "I am B {:b}", i)]
    Binary {
        i: i8,
    },
    #[cfg(feature = "nightly")]
    #[display(fmt = "I am C {}", "_0.display()")]
    Path(PathBuf),
}

#[derive(Display)]
#[display(fmt = "Java EE")]
enum EE {
    A,
    B,
}

#[derive(Display)]
#[display(fmt = "Hello there!")]
union U {
    i: u32,
}

#[derive(Octal)]
#[octal(fmt = "7")]
struct S;

#[derive(UpperHex)]
#[upper_hex(fmt = "UpperHex")]
struct UH;

#[derive(Display)]
struct Unit;

#[derive(Display)]
struct UnitStruct {}

#[derive(Display)]
enum EmptyEnum {}

#[derive(Display)]
#[display(fmt = "Generic")]
struct Generic<T>(T);

#[test]
fn check_display() {
    assert_eq!(MyInt(-2).to_string(), "-2");
    assert_eq!(format!("{:b}", MyInt(9)), "1001");
    assert_eq!(format!("{:o}", MyInt(9)), "11");
    assert_eq!(Point2D { x: 3, y: 4 }.to_string(), "(3, 4)");
    assert_eq!(Error::new("Error").to_string(), "Error");
    assert_eq!(E::Uint(2).to_string(), "2");
    assert_eq!(E::Binary { i: -2 }.to_string(), "I am B 11111110");
    #[cfg(feature = "nightly")]
    assert_eq!(E::Path("abc".into()).to_string(), "I am C abc");
    assert_eq!(EE::A.to_string(), "Java EE");
    assert_eq!(EE::B.to_string(), "Java EE");
    assert_eq!(U { i: 2 }.to_string(), "Hello there!");
    assert_eq!(format!("{:o}", S), "7");
    assert_eq!(format!("{:X}", UH), "UpperHex");
    assert_eq!(Unit.to_string(), "Unit");
    assert_eq!(UnitStruct {}.to_string(), "UnitStruct");
    assert_eq!(Generic(()).to_string(), "Generic");
}

mod generic {
    #[derive(Display)]
    #[display(fmt = "Generic {}", field)]
    struct NamedGenericStruct<T> {
        field: T,
    }
    #[test]
    fn named_generic_struct() {
        assert_eq!(NamedGenericStruct { field: 1 }.to_string(), "Generic 1");
    }

    #[derive(Display)]
    struct AutoNamedGenericStruct<T> {
        field: T,
    }
    #[test]
    fn auto_named_generic_struct() {
        assert_eq!(AutoNamedGenericStruct { field: 1 }.to_string(), "1");
    }

    #[derive(Display)]
    #[display(fmt = "Generic {}", "_0")]
    struct UnnamedGenericStruct<T>(T);
    #[test]
    fn unnamed_generic_struct() {
        assert_eq!(UnnamedGenericStruct(2).to_string(), "Generic 2");
    }

    #[derive(Display)]
    struct AutoUnnamedGenericStruct<T>(T);
    #[test]
    fn auto_unnamed_generic_struct() {
        assert_eq!(AutoUnnamedGenericStruct(2).to_string(), "2");
    }

    #[derive(Display)]
    enum GenericEnum<A, B> {
        #[display(fmt = "Gen::A {}", field)]
        A { field: A },
        #[display(fmt = "Gen::B {}", "_0")]
        B(B),
    }
    #[test]
    fn generic_enum() {
        assert_eq!(GenericEnum::A::<_, u8> { field: 1 }.to_string(), "Gen::A 1");
        assert_eq!(GenericEnum::B::<u8, _>(2).to_string(), "Gen::B 2");
    }

    #[derive(Display)]
    enum AutoGenericEnum<A, B> {
        A { field: A },
        B(B),
    }
    #[test]
    fn auto_generic_enum() {
        assert_eq!(AutoGenericEnum::A::<_, u8> { field: 1 }.to_string(), "1");
        assert_eq!(AutoGenericEnum::B::<u8, _>(2).to_string(), "2");
    }

    #[derive(Display)]
    #[display(fmt = "{} {} <-> {0:o} {1:#x} <-> {0:?} {1:X?}", a, b)]
    struct MultiTraitNamedGenericStruct<A, B> {
        a: A,
        b: B,
    }
    #[test]
    fn multi_trait_named_generic_struct() {
        let s = MultiTraitNamedGenericStruct { a: 8u8, b: 255 };
        assert_eq!(s.to_string(), "8 255 <-> 10 0xff <-> 8 FF");
    }

    #[derive(Display)]
    #[display(fmt = "{} {} {{}} {0:o} {1:#x} - {0:>4?} {1:^4X?}", "_0", "_1")]
    struct MultiTraitUnnamedGenericStruct<A, B>(A, B);
    #[test]
    fn multi_trait_unnamed_generic_struct() {
        let s = MultiTraitUnnamedGenericStruct(8u8, 255);
        assert_eq!(s.to_string(), "8 255 {} 10 0xff -    8  FF ");
    }

    #[derive(Display)]
    #[display(fmt = "{}", "3 * 4")]
    struct UnusedGenericStruct<T>(T);
    #[test]
    fn unused_generic_struct() {
        let s = UnusedGenericStruct(());
        assert_eq!(s.to_string(), "12");
    }
}
