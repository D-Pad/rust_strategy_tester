use colored::Colorize;
use std::collections::HashMap;

pub fn run() {
    // When a keyword ends in !, it means we are invoking a macro. A macro in Rust is a way
    // to define reusable code that can be used to generate code at compile-time It is a
    // powerful feature that allows you to abstract complex logic and reuse it in your code.
    // Examples in the following code are println!() and vec![]

    //////////////////////
    // ==== Tuples ==== //
    //////////////////////
    let point = (3, 4, 5);
    let rgba: (u8, u8, u8, f32) = (100, 255, 20, 0.5);
    let (red, green, blue, alpha) = rgba;
    let data = (10, "hello", (true, 0.1));

    // Tuples must use the access modifier to reference a value rather than []
    // like in Python. Ex: point.0 rather than point[0]
    let rs = red.to_string().red();
    let bs = blue.to_string().blue();
    let gs = green.to_string().green();
    println!("X: {}, Y: {}, Z:{}", point.0, point.1, point.2);
    println!("RGBA: ({}, {}, {}, {})", rs, bs, gs, alpha);
    println!("{} {}", data.2.0, data.1);

    // () this is called the "unit type." It is returned from functions/expressions that
    // don't specify a return value

    //////////////////////
    // ==== Arrays ==== //
    //////////////////////
    // Declaring an array as a literal
    let names = ["Chase", "Alecia", "Rilyn", "Jaylee"];

    // Declaring an array by inferring the type and size
    let mut x: [i32; 5] = [10, 12, 4, 2, 100];

    // Declaring an array of fixed size with default values. The following array will be
    // be filled with 100 instances of 0.00
    let y = [0.00; 100];

    println!("Names: {}, {}", names[0], names[1]);  // Values of names at index 0 and 1
    println!("y[99] = {}", y[99]);                  // Value of y at index 99
    println!("x = {:?}", x);                        // Use {:?} if you want to print the full array
    x.sort();
    println!("x = {:?}", x);

    ///////////////////////
    // ==== Vectors ==== //
    ///////////////////////
    // A vector is just like an array, but they are resizeable, and they are allocated on
    // the heap instead of the stack.
    let mut evens = vec![2, 4, 6, 8];
    let mut odds = Vec::new();
    odds.push(1);
    odds.push(3);
    evens.push(10);

    // We can also create a Vector with a capacity at instantiation
    println!("\nDemonstrating how vector capacity\ncan grow when the defined length is exceeded");
    let mut numbers: Vec<i32> = Vec::with_capacity(4);
    println!("Numbers: {:?}", numbers);
    println!("Numbers len: {}", numbers.len());
    println!("Numbers capacity: {}", numbers.capacity());

    // When a vector grows beyond the size of the predefined capacity, the capacity
    // automatically doubles
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    println!("\nNumbers: {:?}", numbers);
    println!("Numbers len: {}", numbers.len());
    println!("Numbers capacity: {}", numbers.capacity());
    numbers.push(5);  // Capacity doubles once the size exceeds 4.
    println!("\nNumbers: {:?}", numbers);
    println!("Numbers len: {}", numbers.len());
    println!("Numbers capacity: {}", numbers.capacity());

    // Remove the last element of a vector and return that value
    let last = numbers.pop();
    println!("\nLast number: {:?}", last);
    println!("Numbers len: {}", numbers.len());
    println!("Numbers cap: {}", numbers.capacity());

    ////////////////////////
    // ==== HashMaps ==== //
    ////////////////////////
    println!("Hash maps for storing key: value pairs like in Python dictionaries");
    let mut test_hm = HashMap::new();
    test_hm.insert("Open", vec![20]);
    test_hm.insert("High", vec![25]);
    test_hm.insert("Low", vec![18]);
    test_hm.insert("Close", vec![21]);
    println!("Open: {:?}", test_hm["Open"]);

    // Looping through keys and values in a HashMap
    for (key, value) in &test_hm {
        println!("{}: {:?}", key, value);
    }

}