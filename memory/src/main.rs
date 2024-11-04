use std::rt::panic_count::get_count;

static _Y: u32 = 10; // STATIC mem

fn main() {
    println!("Hello, memory!");

    // STACK mem
    let x = 5;
    let z = true;
    let numbers = [1, 2, 3, 4, 5];

    // HEAP mem (get_users isn't defined here)
    let users = get_users();
}

/*

# Static

- Contents:
    - Program binary
    - Static variables
    - String variables
    > Know compilation size

- Size: Fixed

- Lifetime (scope): Whole program

- Cleanup: When program terminates

> Used during compilation

> Memory path and size are fixed, easier to retrieve the value for the program

# Stack

- Contents:
    - Function arguments
    - Local variables
    - Each thread has an isolated stack frame
    > Known compilation size

- Size: Dynamic - Has a top Limit

- Lifetime (scope): Function

- Cleanup: When function returns

> Used during compilation

# Heap

- Contents:
    - Values that live beyond functions
    - Shared across threads
    - Large values
    - Dynamic size values
    > Unknown compilation size

- Size: Dynamic (up to computer limit)

- Lifetime: Defined by programmer or language

- Cleanup: Manually or via GC or via RAII

> Used during runtime because values are dynamic and can't be inferred during compile time
*/