pub fn run() {
    greeting("Hello", "Agata");
    let get_sum = add(10, 40);

    let n3 = 100;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Sum {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}