// References and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts.

// Understanding References
// References: Enable you to borrow values without taking ownership.
// Immutable Reference.
// Mutable Reference.

// Reference created by adidng "&"
fn main(){
    let _x = 5;
    let _r = &_x;
    
    println!("Value of _x is {}", _x);
    println!("Value of _r is {}", _r);
}