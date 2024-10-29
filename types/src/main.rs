fn main() {
    println!("Hello, types!");
}

/*
First 6 are primitives, the Tuple and Array are compounded types

    1. Scalar types
    - Represent a unique value contained inside a known scale
    - Enable the direct compression between values

    1.1 Types
    - int (integer) ex: 5
    - float (floating point) ex: 42.1
    - boolean (bool) ex: true , false
    - character (char) ex: 'a'

    2. Compound types
    - Can aggregate multiple values

    2.1 Types
    - Tuple ex: (5, true, 42.1, 'a')
    - Array ex: [1, 2, 3, 4, 5, 6]

    3. Integers
    | bits  | signed | unsigned |
    | 8     | i8     | u8       |
    | 16    | i16    | u16      |
    | 32    | i32    | u32      |
    | 64    | i64    | u64      |
    | 128   | i128   | u128     |
    | arch  | isize  | usize    | -- varies depending on the architecture

    3.1 signed (positive and negative values)
    - range: -(2 n-1) until 2 n-1 -1
    - i8: -128 until 127

    3.2 unsigned (only positive values since it doesn't store the sign)
    - range: 0 until 2n - 1
    - u8: 0 until 255

    3.3. Literals
    - let y = 199_456_123_9; // same as 1994561239
    - let h = 0xff; // hexa
    - let o = 0o77; // octal
    - let b = 0b1111_0000; // binary
    - let by = b'A'; // byte

    4. Float
    - let x: f64 = 42.1;
    - let y = 42.1; // infers that is a f64 (default float) - f32 is also available but the f64 is the only one that supports double precision

    5. Boolean (bool)
    - let x = true;
    - let y = false;

    6. Char
    - let word: char = 'a'; // emojis also work since we can assign at most 4 bytes in the unicode table

    7. Tuple
    - Can have multiple types of elements inside it

    - let numbers: (i32, i32, f64) = (1, 2, 3.5); // Rust infers the type of each element inside the tuple
    - let (a, b, c) = numbers; // We can do this to easily access each index of the tuple since it has a fixed size
    - let mut numbers = (1, 2, 3); // With mut we can substitute the values within the tuple (numbers.0 = 50 or numbers = (4, 5, 6))

    8. Array
    - Can't have multiple types of elements inside it

    - let numbers: [i32;3] = [1, 2, 3];
    - mutability works the same way as tuples
*/