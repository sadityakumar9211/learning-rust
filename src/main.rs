#![allow(unused)]

use rand::Rng;
use std::{array, cmp::Ordering, f32::consts::PI, fmt::Display, println, slice::Windows};

//  // Illustration 1
// fn main() {
//     println!("What is your name? ");
//     let mut name = String::new();
//     let greeting = "Nice to meet you";

//     io::stdin().read_line(&mut name)
//         .expect("Didn't receive any input");

//     println!("Hello {}!, {}", name.trim_end(), greeting);
// }

// // Illustration 2
// fn main() {
//     println!("Hello, Aditya");
//     const ONE_MIL: u32 = 1_000_000;
//     const PI: f32 = 3.14;
//     let age = "35";
//     let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");

//     age = age + 1;
//     println!("The modified age is {}", age);
// }

// // Illustration 3: Integer and float data types
// fn main() {
//     // singed interger: i8, i16, 132, i64, i128, isize
//     // unsinged interger: u8, u16, u32, u64, u128, usize

//     // usize and isize depends on your machine - for a 64 bit computer, the usize is u64 and so on.

//     // getting the max and min values of all these integer data types

//     println!("The max of {}: {}", "u32", u32::MAX);
//     println!("The min of {}: {}", "u32", u32::MIN); // similarly for all these data types

//     println!("The max of {}: {}", "usize", usize::MAX);
//     println!("The min of {}: {}", "usize", usize::MIN);

//     println!("My computer is {}-bit computer", usize::BITS);

//     // similarly we have floats starting with f --> f32, f64

//     println!("The max of f32: {}", f32::MAX);
//     println!("The min of f32: {}", f32::MIN); // we also have infinity in rust f32::INF

//     let is_true = true;
//     let mut is_false = false;
//     is_false = true;
//     println!("{}, {}", is_false, is_true) // true and false are two boolean values
// }

// // Illustration: 5
// fn main() {
//     // A f32 will have 6 digits of precision
//     // A f64 will have 14 digits of precision

//     // Arithemetic Operation
//     let mut num_1: u32 = 5;
//     let num_2: u32 =  4;

//     // The quotient operator / gives decimal removed value of a division - only integer part.
//     // The remainder operator also is suspected to work the same

// }

// // Illustration 6: Random Number generation
// fn main() {
//     let rand_num = rand::thread_rng().gen_range(1..100);
//     println!("Random Number: {}", rand_num)
// }

// // Illustration 7: If operators
// fn main() {
//     let age = 8;
//     let imp_birth = "Important Birthday!";
//     let not_imp_birth = "Not Important Birthday!";

//     if (age >= 1) && (age <= 18) {
//         println!("{}", imp_birth);
//     } else if age == 21 {
//         println!("{}", imp_birth);
//     } else if age >= 65 {
//         println!("{}", imp_birth);
//     } else {
//         println!("{}", not_imp_birth);
//     }
// }

// // Illustration 8: Kind of Ternary Operator
// fn main() {
//     let mut age = 57;

//     let can_vote = if age >= 18 { true } else { false };

//     println!("{}", can_vote);
// }

// // Illustration 9: match - another conditional
// fn main() {
//     let age = 32;
//     match age {
//         1..=18 => println!("Important Birthday"),       // range with end point included
//         21 | 50 => println!("Important Birthday!"),    // this is union or
//         65..=i32::MAX => println!("Important Birthday!"),
//         _ => println!("Not an Important Birthday!"),
//     };
// }

// Ill: More on match - using Ordering

// fn main() {
//     let my_age = 18;
//     let voting_age = 18;

//     match my_age.cmp(&voting_age) {
//         Ordering::Less => println!("Can't Vote"),
//         Ordering::Greater => println!("Can Vote"),
//         Ordering::Equal => println!("You gained the right to vote!"),
//     };
// }

// // Illustration 10: Arrays - homogeneous + fixed size
// fn main() {
//     let array1 = [1, 2, 3, 4, 6, 7, 8, 9, 0];
//     println!("first: {}", array1[0]); // accessing the elements using index - Rust is 0 indexing based.
//     println!("The length of the array1: {}", array1.len()); // .len function gets you the length of the array

//     // looping through the array indices:
//     let mut loop_idx = 0;

//     loop {
//         if array1[loop_idx] % 2 == 0 {
//             loop_idx += 1;
//             continue;
//         } else if array1[loop_idx] == 9 {
//             break;
//         }
//         println!("Val: {}", array1[loop_idx]);
//         loop_idx += 1;
//     }

//     // while loop
//     println!("Using while loop\n");
//     let mut index = 0;
//     while index < array1.len() {
//         println!("Val: {}", array1[index]);
//         index += 1;
//     }

//     // for loop
//     println!("Using For loop\n");
//     for val in array1.iter() {
//         println!("Val = {}", val);
//     }
// }

// Illustration 11: Tuples - heterogeneous + fixed size

// fn main() {
//     let my_tuple: (u8, String, &str, i32, f64) =
//         (47, String::from("Aditya"), "Kumar", 45, 48.89898);
//     println!("First: {}", my_tuple.0); // to access tuple elements, use .index operator

//     // destructuring the tuple
//     let (v1, v2, v3, v4, v5) = my_tuple;
//     println!("{},{},{},{},{}", v1, v2, v3, v4, v5);
// }

// // Illustration 12: Strings
// fn main() {
//     let my_str = "Hello";
//     let mut ano_str = String::new(); // creates a new empty string   --> reference to a string -> point to a string in memory
//     ano_str = " World!".to_string();      // this is a vector of bytes
//     println!("{}, {}", my_str, ano_str);

//     ano_str.push('A');
//     ano_str.push('B');
//     println!("Modified String: {}", ano_str);

//     let mut greetings = "Nice to meet you!";

//     // printing the words by splitting
//     for word in greetings.split_whitespace() {
//         println!("{}", word);
//     }

//     // replacing character in strings
//     ano_str = ano_str.replace("W", "Another");
//     println!("{}", ano_str);
// }

// // Illustration 13: Creating a String from multiple different characters
// fn main() {
//     let st3 = String::from("c v f g h r y e w");
//     let mut v1: Vec<char> = st3.chars().collect();

//     v1.sort();
//     v1.dedup();

//     for char in v1 {
//         println!("{}", char);
//     }

//     // creating a string literal
//     let st4 = "Random String";
//     let mut st5: String = st4.to_string(); // heap allocated string (resizable strings)
//     println!("{}, {}", st4, st5);

//     // converting an string to an array of bytes

//     let byte_arr = st5.as_bytes();
//     let st6 = &st5[0..6]; // getting the slice of the string (range is specified usign .. operator)

//     println!("{}, {}", st5, st6);

//     // priting the byte array
//     for byte in byte_arr.iter() {
//         println!("Val: {}", byte);
//     }

//     st5.clear();

//     // combining the strings

//     let st7  = String::from("Just Some");
//     let st8 = String::from(" Words");

//     let st9 = st7.clone() + &st8;    // if not cloned, it is moved
//     println!("{},{},{}", st9, st8, st7);

//     for char in st9.bytes() {
//         println!("{}", char);
//     }
// }

// // Illustration 13: Casting
// fn main() {
//     let int1_u8 = 5u8;
//     let int_2_u8 = 10_u8;

//     let int_u32: u32 = (int1_u8 as u32) + (int_2_u8 as u32);

//     println!("{}", int_u32);
// }

// // Illustration 14: Enum data types
// fn main() {
//     enum Day {
//         Monday,
//         Tuesday,
//         Wednesday,
//         Thursday,
//         Friday,
//         Saturday,
//         Sunday,
//     }

//     impl Day {
//         // &self means the reference to Day type as it is defined inside impl Day
//         fn is_weekend(&self) -> bool {
//             match self {
//                 Day::Saturday | Day::Sunday => true,
//                 _ => false,
//             }
//         }
//     }

//     let today = Day::Sunday;

//     match today {
//         Day::Monday => println!("It sucks"),
//         _ => println!("At least not Monday!"),
//     }

//     println!("Is today a weekend: {}", today.is_weekend());
// }

// // Illustration 15: Vectors -> homogeneous + growable if defined mutable
// fn main() {
//     // empty vector
//     let mut vec1 = Vec::new();
//     vec1.push(3);

//     // vector with defined values
//     let mut vec2: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
//     vec2.push(8);
//     println!("First: {}", vec2[0]);

//     let second: &i32 = &(vec2[1] as i32);

//     match vec2.get(1) {
//         // if this iterator is some value then there would be second
//         Some(second) => println!("2nd: {}", second),
//         // if this iterator gives no value, then we don't have second.
//         None => println!("No second value"),
//     }

//     for i in &mut vec2 {
//         *i *= 2;
//     }

//     for i in &vec2 {
//         println!("{}", i)
//     }

//     println!("The length of vec2 is: {}", vec2.len());
//     println!("Pop the last value: {:?}", vec2.pop());
//     println!("The length of vec2 is: {}", vec2.len());

//     println!("The vec2 is {:#?}", vec2);
// }

// //Illustration 16: Functions
// fn say_hello(name: &str) {
//     // functions can be defined before or after the main function.
//     println!("Hello {}", name);
// }

// fn get_sum(x: i32, y: i32) -> i32 {
//     // x + y
//     return x + y;
// }

// fn get_2(x: i32) -> (i32, u32) {
//     return (x + 1, (x + 2) as u32);
// }

// fn get_arr_sum(vec: [i32; 4]) -> u32 {
//     let mut sum = 0;
//     for i in vec {
//         sum += i;
//     }
//     return sum as u32;
// }

// fn get_vec_sum(vec: &Vec<i32>) -> i32 {
//     let mut sum = 0;

//     for i in vec.iter() {
//         sum += i;
//     }
//     return sum;
// }

// fn main() {
//     say_hello("Aditya");

//     let sum = get_sum(4, 5);
//     println!("The sum is : {}", sum);

//     let (v1, v2) = get_2(5);
//     println!("v1: {}\nv2: {}", v1, v2);

//     let arr = [1, 2, 3, 4];
//     println!("The sum of array values are: {}", get_arr_sum(arr));

//     let vec = vec![1, 2, 3, 4, 5];

//     println!("The sum of elements are: {}", get_vec_sum(&vec));
// }

// // Illustration 17: Generics
// use std::ops::Add;

// fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }

// fn main() {
//     println!("5+4: {}", get_sum_gen(5, 4));
//     println!("5+4: {}", get_sum_gen(5.9, 4.2));
// }

// // Ownership
// //1. Each value is going to have a variable and that is the owner of that value.
// //2. There is only one owner for a value at any time.
// //3. Whenever the owner goes out of the scope, the value is going to disappear.

// fn main() {

//     // This is not going to apply for datatypes: integers, floats, chars, tuples, booleans,  but apply for
//     // vectors, Strings, arrays, etc.
//     let str1: String = String::from("World");
//     let str2: String = str1.clone();
//     // let str2: String = str1;
//     println!("Hello {}", str1);
// }

// // Continued
// fn print_str(x: String) {
//     println!("A string here: {}", x);
// }

// fn print_return_str(x: String) -> String {
//     println!("A string here: {}", x);
//     x
// }

// fn change_str(name: &mut String) {
//     name.push_str("Helo");
//     println!("{}", name);
// }

// fn main() {
//     let str1 = print_return_str(String::from("Hello"));
//     print_str(String::from("World"));

//     println!("The returned string: {}", str1);

//     change_str(&mut String::from("Hello"));
// }

// // Hashmaps
// use std::collections::HashMap;
// fn main() {
//     let mut heroes = HashMap::new();
//     heroes.insert("Superman", "Clark Kent");
//     heroes.insert("Batman", "Bruce Wayne");
//     heroes.insert("The Flash", "Barry Allen");

//     println!("{:#?}", heroes);

//     // now traversing through the elements of the hashmap
//     for (k, v) in heroes.iter() {
//         println!("{} -> {}", k, v);
//     }

//     println!("The length of the hashmap is: {}", heroes.len());

//     if heroes.contains_key(&"Batman") {
//         let the_batman = heroes.get(&"Batman");
//         match the_batman {
//             Some(x) => println!("Batman is a hero"),
//             None => println!("Batman is not a hero yet"),
//         }
//     }
// }

// // structs
// fn main() {
//     struct Customer {
//         name: String,
//         address: String,
//         balance: f32,
//     }

//     let mut bob = Customer {
//         name: String::from("Bob Smith"),
//         address: String::from("4 London"),
//         balance: 500_000f32,
//     };

//     bob.address = String::from("555 Main St.");
// }

// // structs
// fn main() {
//     // tying struct properties to functions using traits
//     trait Shape {
//         fn new(length: f32, width: f32) -> Self;
//         fn area(&self) -> f32;
//     }

//     struct Rectangle {
//         length: f32,
//         width: f32,
//     };
//     struct Circle {
//         length: f32,
//         width: f32,
//     };

//     // now implementing trait for these structs

//     impl Shape for Rectangle {
//         fn new(length: f32, width: f32) -> Rectangle {
//             return Rectangle { length, width };
//         }

//         fn area(&self) -> f32 {
//             return self.length * self.width;
//         }
//     }

//     // implementing trait for circle
//     impl Shape for Circle {
//         fn new(length: f32, width: f32) -> Circle {
//             return Circle { length, width };
//         }

//         fn area(&self) -> f32 {
//             return (self.length / 2.0).powf(2.0) * PI;
//         }
//     }

//     let rec = Rectangle::new(14f32, 10f32);
//     let circle = Circle::new(4f32, 5f32);

//     println!("The area of rectangle: {}", rec.area());
//     println!("The area of circle: {}", circle.area());
// }

// // GIVING ERRORS - IN MODULES
// mod resturant;
// use crate::resturant::order_food;
// fn main() {
//     // Crates: Modules that produce a library or executables
//     // Modules: Organize and handle privacy
//     // Packages: Build, test, and share creates,
//     // Paths: A way of naming an item such as a struct, function

//     order_food()
// }

fn main() {
    println!("Hello, World!!");
}
