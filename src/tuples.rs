pub fn run() {
    // Max 12 elements

    let person: (&str, &str, i8) = ("Kamil", "Nowak", 24);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

    println!("{:?}", person);
}