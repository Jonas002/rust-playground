/*

INTEGERS
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize

LITERALS
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize

SCALAR TYPES
signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
floating point: f32, f64
char -> Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
bool -> either true or false
and the unit type () -> whose only possible value is an empty tuple: ()

COMPOUND TYPES
arrays like [1, 2, 3]
tuples like (1, true)

*/

// shorten imports with "use"
use std::mem;

pub fn run(){

    let x = 1; // default i32
    let y = 2.5; // default f64
    let y: i64 = 424242424; // add explicitly
    let a = 'c'; // char
    let abc = '\u{1F600}'; // emoji

    println!("Max i32: {}. Max i64: {}", std::i32::MAX, std::i64::MAX);

    let is_active: bool = true;

    let is_greater: bool = 10 > 5;

    println!("{:?}", (x, y, is_active, is_greater, a, abc));

    let mut hello = String::from("hello"); // string

    println!("Length: {}", hello.len());
    hello.push('w'); // push char
    println!("{}", hello);
    hello.push_str("old"); // push string
    println!("{}", hello);
 
    println!("{}", hello.capacity()); // check capacity
    println!("{}", hello.is_empty()); // check if is empty
    println!("{}", hello.contains("hello")); // check if contains
    println!("{}", hello.replace("hello", "hiho asdf ")); // check and replace
    
    hello = hello.replace("hello", "hiho asdf ");

    for word in hello.split_whitespace() {// split whitespace
        println!("{}", word ); 
    }

    // create string with capacity

    let mut s = String::with_capacity(10);
    s.push('s');
    s.push('a');


    assert_eq!(2, s.len()); // assertion testing
    assert_eq!(10, s.capacity()); // assertion testing
    println!("{}", s);


    // tuples max 12 elements
    let person: (&str, &str, i8) = ("ddasd", "Hess", 33);
    println!("Tuples: {} is from {} and is {}", person.0, person.1, person.2);

    // arrays
    let numbers: [i32;5] = [1,2,3,4,5];
    println!("Array: {:?}", numbers);
    println!("Single value: {}", numbers[0]);
    println!("Array Length and Memory: {} len and {} bytes", numbers.len(), mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..2];
    println!("Slice: {:?}", slice);

    // Vectors
    let mut vnumbers: Vec<i32> = vec![1,2,3,4,5];
    println!("Vector: {:?}", vnumbers);
    vnumbers.push(6);
    println!("Single value: {}", vnumbers[0]);
    println!("Vector Length and Memory: {} len and {} bytes", vnumbers.len(), mem::size_of_val(&vnumbers));

    // Get slice
    let vslice: &[i32] = &vnumbers[1..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for vn in vnumbers.iter(){
        println!("Vector Number: {}", vn);
    }

    // Loop through vector values and mutate
    for vn in vnumbers.iter_mut(){
        *vn /= 2;
        println!("Mutated Vector Number: {}", vn);
    }

}