use rand::Rng;

pub fn run() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Number {}", count);

        if count == 20 {
            break;
        }
    }

    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }

    for x in 0..1000 {
        println!("{}", x * x * x);
    }

    //loops can return a value
    let result = loop {
        let rand_value = rand::thread_rng().gen_range(0..240);
        if rand_value > 120 && rand_value < 150 {
            break rand_value;
        }
    };

    println!("returned value fro loop {}", result);
}