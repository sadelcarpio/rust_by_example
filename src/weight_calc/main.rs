use std::io;

fn main() {
    println!("Enter your weight! (kg)");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();  // if there is an error, unwrap terminates the program
    let weight: f32 = input.trim().parse().unwrap();
    println!("Input: {}", weight);
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {} kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711  // is implicitly returned
}

// Ownership:
// 1. Each value in Rust is owned by a variable.
// 2. when the owner goes out of scope, the value will be deallocated.
// 3. There can only be ONE owner at a time.


// mutable borrow:
// 1. Only one mutable borrow can exist at a time for a specific piece of data.
// 2. No immutable borrows can exist while a mutable borrow is active.

// immutable borrows:
// 1. Multiple immutable borrows can exist concurrently for the same data, as long as there is no mutable borrow active.
// 2. They don't modify the data, so sharing them freely is safe.
