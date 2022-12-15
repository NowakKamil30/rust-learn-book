pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    numbers[2] = 100;
    numbers.push(1000);
    numbers.pop();
    println!("{:?}", numbers);
    println!("{}", numbers[0]);
    println!("{}", numbers.len());

    // Stack allocated
    println!("{}", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);

    for x in numbers.iter() {
        println!("Number {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    for x in numbers.iter() {
        println!("Number x2 {}", x);
    }
}