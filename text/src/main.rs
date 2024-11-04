fn main() {
    println!("Hello, world!");

    // chars or array of chars
    let l0 = 't';
    let l1 = 'e';
    let l2 = 's';
    let l3 = 't';
    let l4 = 'e';
    let test = ['t', 'e', 's', 't', 'e'];

    println!("{l0}{l1}{l2}{l3}{l4}");
    println!("{:?}", test);

    // literal string - immutable - these strings are stored inside code segment in the static memory during compilation
    let literal = "teste";

    println!("{literal}");

    // Heap String - Dynamic String - String
    let mut s = String::new();
    // s.push('t');
    // s.push('e');
    // s.push('s');
    // s.push('t');
    // s.push('e');
    s.push_str("hello");
    s.push(' ');
    s.push_str("test!");
    println!("{}", s);

    let s2 = literal.to_string();
    println!("{}", s2);

    let s3 = s2.clone();
    println!("{}", s3);

    let s4 = String::from_iter(test); // from array
    println!("{}", s4);

    // Internal trait will convert literal value into() the heap string type, type needs to be explicitly defined in this case
    let s5: String = "TEST".into();
    println!("{s5}");

    let s6 = String::from(" TEST😅\n");

    // This prints 8, even though we have 5 chars, why's that?
    // Because len returns the bytes of the String, hence we have 4 bytes for test and the emoji has 4 bytes
    // > trim() removes not only empty spaces but also removes special chars such as \n
    println!("{}", s6.trim().len());

    // To print the actual amount of chars (5) we can do the following:
    println!("{}", s6.trim().chars().count());

    // println!("{}", "-".repeat(10)); // Simple repeat
    // println!("{:-<40}", "Fim"); // Repeat with the word in the left
    // println!("{:->40}", "Fim"); // Repeat with the word in the right
    println!("{:-^40}", "Fim"); // Repeat with the word in the middle
}

/*

Primitive type

- Char: Characters with a max of 4 bytes that are valid from the unicode table

Other types

- Literal/Static strings (&str is the type which means String Slice and the & is the reference/pointer indicator):
  Good for static/constants since it will store the static value in a hexadecimal compiled file that holds all strings of this type
- Heap String or String: Dynamic string that can be used on most cases
*/