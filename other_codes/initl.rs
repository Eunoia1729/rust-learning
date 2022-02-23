fn foo() {
    let x = Box::new(42);
    println!("x points to {}", x);
    println!("x points to {}", *x);
}

fn foo2() {
    let x = Box::new(75);
    let y = Box::new(42);
    // x = y;         // Not allowed, x is immutable.
    // *x = 43;       // Not allowed, *x is immutable.
    let mut x = Box::new(75);
    let z = y;            // OK, x is mutable.
    println!("z is {}", z);
    *x = 43;          // OK, *x is mutable.
}

// fn foo3() {
//     let x = Box::new(12);
//     let mut y = x;
//     y = Box::new(22);
//     println!("y is {}", x);
// }
fn foo4() {
    let x = 3;
    let y = x;
    println!("x is {}", x);

}
fn main() {
    // foo();
    // foo2();
    // foo3();
    foo4();
}