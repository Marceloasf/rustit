const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const SUM: u32 = 1 + 1; // non-constant values such as X can't be used to assign a value to a const
    println!("The constants are SUM: {SUM} and THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;
    let x = x + 1;

    { // Start intern scope
        // X inside this scope is different from X outside this scope, resulting in a 12 here and 6 outside
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    } // End intern scope -- Clear variables from it

    println!("The value of x is: {x}"); // x still the same because the value is from the outside scope
    let _y = 0; // Add _ as prefix to ignore things in rust
}
