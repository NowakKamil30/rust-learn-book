pub fn run() {
    let mut hello = String::from("Hello");

    println!("Length: {}", hello.len());
    hello.push('W');
    hello.push_str(" sadsad");
    println!("Length: {}", hello.len());

    println!("Capacity: {}", hello.capacity());

    println!("Contains 'World' {}", hello.contains("World"));

    println!("Replace {}", hello.replace("Hello", "World"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(100);

    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(100, s.capacity());
}