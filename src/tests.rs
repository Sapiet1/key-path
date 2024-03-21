use super::*;
use std::{fmt::Debug, hint};

#[test]
fn test_new() {
    struct Struct(i32, i32);

    let path_zero = key!(Struct[0]);
    let path_one = key!(Struct[1]);

    let _ = hint::black_box(path_zero);
    let _ = hint::black_box(path_one);
}

#[test]
fn test_new_reference() {
    struct Struct<'a>(&'a i32);

    let hidden = 4;
    let instance = Struct(&hidden);
    let path = key!(Struct[0]);

    assert_eq!(instance.0, *instance.enter(path));
}

#[test]
fn test_new_generic() {
    struct Struct<T>(T);

    fn _test<T>(instance: Struct<T>, compare: T)
    where
        T: PartialEq + Copy + Debug,
    {
        let path = key!(Struct<T>[0]);
        assert_eq!(*instance.enter(path), compare);
    }

    let instance = Struct(44);
    _test(instance, 44);
}

#[test]
fn test_enter() {
    struct Struct {
        field: usize,
    }

    let instance = Struct { field: 32 };
    let path = key!(Struct[field]);

    assert_eq!(instance.field, *instance.enter(path));
}

#[test]
fn test_enter_mut() {
    struct Struct {
        value: i32,
    }

    let mut instance = Struct { value: 3 };
    let path = key!(Struct[value]);

    *instance.enter_mut(path) -= 10;
    assert_eq!(instance.value, -7);
}

#[test]
fn test_append() {
    struct Inner(#[allow(unused)] u128, u128);
    struct Struct(Inner);

    let mut instance = Struct(Inner(0, 22));

    let inner_path = key!(Inner[1]);
    let outer_path = key!(Struct[0]);

    // Comiler Error: Paths work correctly.
    // let path = inner_path.append(outer_path);

    let path = outer_path.append(inner_path);
    *instance.enter_mut(path) += 40;

    assert_eq!(instance.0 .1, 62);
}

#[test]
fn test_nested() {
    struct Inner(#[allow(unused)] u64, u32);

    struct Struct {
        value: Inner,
    }

    let mut instance = Struct {
        value: Inner(4, 10),
    };

    let path = key!(Struct[value][1]);
    *instance.enter_mut(path) += 200;

    assert_eq!(instance.value.1, 210);
}
