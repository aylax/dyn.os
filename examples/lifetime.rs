// lifetime.rs

fn main() {
    life_alphas(&1, &2, &3);
    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);
    println!("lifetime.rs");
}

fn life_alphas<'a, 'b, 'c>(a: &'a i32, b: &'b i32, c: &'c i32) {
    println!("a is {} and b is {} and c is {}", a, b, c);
}

use std::fmt::Debug;
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T: Debug>(t: T) {
    println!("`print`: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}


