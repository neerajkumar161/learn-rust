use std::io;

fn main() {
    /*
     * Data Types in Rust
     * https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
     *
     * Scalar and Compound
     *
     * Scalar - Types that represents single value
     * Integers, Floating-point numbers, Booleans and Characters
     *
     *
     * Integers
     * Signed -> i8, i16, i32, i64, i128, isize (based on OS architecture)
     * Unsigned -> u8, u16, u32, u64, u128, usize (based on OS architecture)
     *
     * Integer Overflow - https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#integer-overflow
     */

    let num1: i32 = -78;
    let num2: u32 = 90;
    println!("num1: {num1} and num2: {num2}");

    /* Floating Point Number
     *
     * Two Floating Point Types
     * f32 and f64 (f64 is default)
     *
     */
    let num3: f32 = 34.89;
    let num4 = 78.98;
    println!("Floating point numbers: num3: {num3}, num4: {num4}");

    /* Boolean Types
     * true and false
     *
     */
    let bool_true = true;
    let bool_false = false;
    println!("Boolean values: {bool_true}, {bool_false}");

    /* Character Types
     * Rust’s char type is the language’s most primitive alphabetic type.
     */
    let char1 = 'z';
    let char2 = "Z";
    let char3 = '😻';
    println!("Character Types: {char1}, {char2}, {char3}");

    /* Compound Types - They can group multiple values into one type.
     *
     * Two Primitives data types
     * Tuples and Arrays
     *
     *
     * Tuple Type - [Similar to Typescript Tuple]
     * Grouping varities of types into one compound type.
     * Fixed length, they cannot grow or shrink.
     */

    let tuple = (500, 6.4, false);
    println!("Tuple First value: {:?}", tuple.0);
    println!("Tuple: {:?}", tuple);

    // Get specific value from tuple, DESTRUCTURING
    // let (a, b, c) = tuple;

    /* Arrays
     * Elements should be of same type.
     * Fixed length
     */
    let arr = [1, 2, 3, 4, 5, 6];
    println!(
        "Array: First element: {:?}, Complete Array: {:?}",
        arr[0],
        { arr }
    );
    // Define array of numbers having length 4
    let arr2: [i32; 4] = [1, 2, 3, 4];
    println!("Array 2: {:?}", arr2);

    /* Invalid array element access - Out of Bound Index */
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element = arr[index];

    println!("The value of the element at the index {index} is: {element}");
}
