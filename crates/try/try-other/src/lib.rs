#![feature(try_trait_v2)]
#![allow(dead_code, warnings)]
#![feature(unboxed_closures)]
#[cfg(test)] // this crate is for trying stuffs
mod try_atomic;
mod try_control_flow;
mod try_diff_text;
mod try_eq;
mod try_fat_pointer;
mod try_fn;
mod try_monad;
mod try_mut_ref;
mod try_profile;
mod try_random;
mod try_rayon;
mod try_ref;
mod try_serde;
mod try_size;
mod try_trait;
mod try_whimsical;
mod try_xxhash;

use husky_check_utils::*;
use husky_print_utils::*;

#[derive(Debug)]
struct X {
    a: i32,
    b: i32,
}

fn f() {
    let mut x = X {
        a: todo!(),
        b: todo!(),
    };
    let b = &mut x.a;
    {
        let a = &mut x.b;
        *a = 1;
    }
    *b = 1;
}

fn g() {
    let mut x = 1;
    let a = &mut x;
    &x;
    let a = &mut x;
    *a = 1;
}

fn h() {
    let mut x1 = X { a: 1, b: 2 };
    let xs1: Vec<&mut X> = vec![&mut x1];
    let mut x2 = X { a: 1, b: 2 };
    let xs2: Vec<&mut X> = vec![&mut x2];
    // same(&xs1, &xs2);
    if true {
        same(&xs1, &xs2)
    } else {
        let c = &mut x2;
    }
    println!("{:?}", xs1);
    // println!("{:?}", xs2);
    // println!("{:?}", xs1)
}

fn same<'a>(xs1: &Vec<&'a mut X>, xs2: &Vec<&'a mut X>) {}

fn mv(x: X) {}

struct B2(String);
struct B3(X);

// #[test]
// fn it_works() {
//     let a = String::new();
//     let b = B2(a);
//     println!("{:?}", a)
// }
