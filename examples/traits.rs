// traits.rs

trait A {
    fn get(&self) -> u8;
}

trait B {
    fn get(&self) -> bool;
}

struct S { a:u8, b: bool }

impl S {
    fn new(a: u8, b: bool) -> Self {
        Self { a, b }
    }
}

impl A for S {
    fn get(&self) -> u8 { self.a }
}

impl B for S {
    fn get(&self) -> bool { self.b }
}


fn main() {
    let s = S::new(12, true);
    let a = <S as A>::get(&s);
    let b = <S as B>::get(&s);
    println!("a is {}", a);
    println!("b is {}", b);
}
