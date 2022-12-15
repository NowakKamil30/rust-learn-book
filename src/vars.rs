pub fn run() {
    let name = "Brad";
    let mut age = 24;

    age = 25;

    println!("My name is {} and I am {}", name, age);

    // const
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Kamil", 24);
    println!("My name is {} and I am {}", my_name, my_age);

    const COUNT: u32 = 100_000; //cannot be mutable, we cannot return it and we have to set the type of const.
    // default i32
    let a = 92; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_00; // Binary
    let e = b'A'; // Byte (u8 only)


}