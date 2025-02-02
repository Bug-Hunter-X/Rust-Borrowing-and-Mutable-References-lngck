fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y;    // z is an immutable reference to y
    *y = 10;       // Modify x through y
    println!("x = {}", x); // Prints x = 10

    // let z = &mut x; // This will cause a compile-time error
    // because x is already mutably borrowed by y
}