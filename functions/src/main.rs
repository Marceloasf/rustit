fn say_hello(name: &str, color: &str) {
    println!("Hello, {}. Your color is {}!", name, color);
}

fn add_numbers(x: i32, y: i32) -> i32 {

    // this is a early return statement, because of that we need to use return
    if x == 0 {
        return y;
    }

    x + y // don't need the return, just make sure you don't use ;
}

fn print_doubles() {
    let input = "56 65 58 47 59 54 21 50";

    let result: Vec<i32> = input
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .map(|n| n * 2)
        .collect();

    println!("{:?}", result)
}

fn main() {
    let z = say_hello("world", "red");
    say_hello("x", "blue");

    // returns a unit type since there's no null in rust
    println!("{:?}", z);

    // New inner scope is an expression
    let y = {
        say_hello("y", "red");
        let _x = 5;
        99 // no ; to define this as the return of the expression
    };

    println!("{}", y);

    let res = add_numbers(3, 4);
    println!("{} + {} = {}", 3, 4, res);

    println!("{}", add_numbers(0, 9));

    print_doubles();
}

/*

Expressions (creates a value like a supplier, we don't need a return statement):
- let x = 5;
- let x = vec![];
- let x = say_hello("Bruno");
- let x = 5 * 5;
- let x = 10 > 5;



*/