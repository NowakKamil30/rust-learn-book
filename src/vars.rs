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
}