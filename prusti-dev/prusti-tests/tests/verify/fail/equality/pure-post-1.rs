use prusti_contracts::*;

#[derive(Clone,PartialEq,Eq)]
struct A {
    i: i32,
}

#[pure] 
#[requires(_x == _y)]
#[ensures(old(_x) == old(_y))]
fn get_value(_x: A, _y: A) -> A { //~ ERROR return type of pure function does not implement Copy
    _x
}

fn main() {
}
