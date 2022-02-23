fn foo() {
    let x = 5;
    let mut xr = &x;        // Ok - x and xr have the same lifetime
    {
        let y = 6;
        xr = &y;            // Error - xr will outlive y
        println!("{}", xr);
    }                       // y is released here
    // println!("{}", xr);   // xr is used here so it outlives y. Try to comment out this line.
}                           // x and xr are released here

fn main() {
    foo();
}