use std::io;

fn main() {
    let a = 5;
    let b = a;  // is valid because it copies the actual value (its size is known)
    let mut input = String::new();  // String is a smart pointer (size is not known at compile time)
    let s1 = &input;
    let s2 = &input;  // can only have a single mutable borrow at a time (no other borrows), but many inmutable borrows
    println!("{} {}", s1, s2);
    // some_fn(input);  // ownership moves to s on the function. The String is deallocated when some_fn finishes
    some_fn(&mut input);  // passing the reference
    // let mut s = input;  // points to the same string, can lead to memory corruption. Ownership is moved to s.
    io::stdin().read_line(&mut input);  // here is borrowed as mutable, since there are no more immutable borrows used.
    let mut mars_weight: f32 = calculate_weight_on_mars(100.0);  // define mutable
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {} g", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711  // is implicitly returned
}

fn some_fn(s: &mut String) {
    s.push_str("a");
}  // reference: allows to use a variable without taking ownership. &mut: mutable ref

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
