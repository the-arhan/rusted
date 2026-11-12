// Ownership, Borrowing and References

// Ownership

// C, C++ -> Memory Managment Control Issue
// Garbage Collector solved this issue, but created a new issue.
// -> Slow Performance:
// [Stopping/Resuming the program]

// Ownership Rules 
// 1. Each value in Rust has a variable that's it's owner.\

// fn main() {
//     let s1 = String::from("Rust");
//     let len = calculate_length(&s1);
//     println!("Length of '{}', is {}.", s1, len);

// }

// fn calculate_length(s: &String) -> usize{
//     s.len()
// }

// 2. There can be only one owner at a time.

// fn main() {
//     let s1 = String::from("Rust");
//     let s2 = s1;
//     println!("{}", s2)
// }

// 3. When the owner goes out of scope, the value will be dropped.

// fn main() {
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1);
//     println!("Length of '{}' is {}.", s1, len)

// }
// // s1 goes out of scope and its value will be dropped.

// fn printlost(s: &String) {
//     println!("{}", &s1)
// }

// fn calculate_length(s: &String) -> usize{
//     s.len()
// }