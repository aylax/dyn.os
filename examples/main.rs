fn main() {
    let a: u8 = 12;
    let mut b: u8;
    for item in 1..4 {
        b = item;
        println!("{} {} {}", item, a, b);
    }
    let c = Container(3, 4);
    let (d, e) = (3, 4);
    let r = c.contains(&d, &e); 
    println!("{:?} contains ({}, {}): {}", c, d, e, r);
}

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;        
}

#[derive(Debug)]
struct Container(i32, i32);

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, a: &i32, b: &i32) -> bool {
        (&self.0 == a) && (&self.1 == b)
    }
}

