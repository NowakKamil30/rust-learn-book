pub fn run() {
    //Print
    println!("Hello from print.rs file");

    // Basic Formatting
    println!("Number {}", 1);
    println!("{}", 2);

    // Position Arguments
    println!("Number {0} and {1} and again {0}", 2, 10);

    // Named Arguments
    println!("Number {one} and {two} and again {one}", two = 2, one = 10);

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));
}