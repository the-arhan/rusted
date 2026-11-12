// Primitive data types-  
// 
//     int, float, bool, char
// 
// Integer -
//     Rust has signed (+ and -) and unsigned integer (only +)
//     types of different sizes.
// 
//     i8, i16, i32, i64, i128: Signed integers.
//     u8, u16, u32, u64, u128: Unsigned integers.
// 
//     

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;

    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

// diff b/w i32() (can take upto 32 bits) and i64() (upto 64 bits)
// range: 
//      i32: 2147483647
//      i64: 9223372036854775807

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);


// Floats (FLoating Point Types) - 
//      f32, f64

    let pi: f64 = 3.14;
    println!("vlaue of pi: {}", pi);

// Boolean Values: true, false
    let is_snowing: bool = true;
    println!("It's {} that it's snowing!", is_snowing);

// Character Type: char - 
    let letter: char = 'a';
    println!("{} is the first letter of the alphabet.", letter);

// Compound data types - 
//      arrays, tuples, slices, and strings (Slice string)

// Arrays
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);
    // let mix = [1,2,"apple",true];
    // println!("Mix: {:?}", mix); (Won't work because array can only contain homogenous datatypes.)

   let fruits: [&str; 3] = ["Apple", "Banana", "Carrot"];
   println!("Fruits Array: {:?}", fruits);
   println!("Fruits Array 1st element: {}", fruits[0]);
   println!("Fruits Array 2nd element: {}", fruits[1]);
   println!("Fruits Array 3rd element: {}", fruits[2]); 

// Tuples - 
   let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
   println!("Human Tuple: {:?}", human);

   let my_mix_tuple = ("kratos", 23, true, [1,2,3,4,5]);
   println!("My Mix Tuple: {:?}", my_mix_tuple);

// Slices - 
//      [1,2,3,4,5]

    let num_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", num_slices);

    let animal_slice: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", animal_slice);

    let book_slice: &[&String] = &[&"IT".to_string(), &"ZEN".to_string(), &"THE ART OF SEDUCTION".to_string()];
    println!{"Book Slice: {:?}", book_slice};


// Strings Vs String Slices (&str) -
//       Strings [growable, mutable, owned string type]     

    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold says: {}", stone_cold);

//       &str (String Slices)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);

}