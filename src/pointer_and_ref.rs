use std::mem::take;

pub fn run() {
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    //With non-primitives, if you assign another variable to a piece of data,
    // the first variable will no longer hold the value. You'll need to
    // use a reference & to point to the resource

    let vec1 = vec!(1,2,3);
    let vec2 = &vec1;

    println!("Values {:?}", (&vec1, vec2));

    println!("Values {:?}", (arr1, arr2));

    let s1 = String::from("lalal");
    let s2 = s1.clone();

    println!("{}", s1);
    take_ownership(s2);
    //println!("{}", s2);
    let s3 = takes_and_gives_back(s1);
    // println!("{}", s1);
    println!("{}", s3);
    borrow_ownership(&s3);
    println!("{}", s3);
    let mut s4 = s3.clone();
    borrow_and_change_data(&mut s4);
    println!("{}", s4);
    scope();
    string_slice(& s4);

}

fn take_ownership(some_string: String) {
    println!("{}", some_string)
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn borrow_ownership(some_string: &String) {
    println!("borrow {}", some_string)
}

fn borrow_and_change_data(some_string: &mut String) {
    some_string.push_str("1111");
    println!("borrow and change {}", some_string);
}

fn scope() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s; // nie mozna tutaj uzyć ponieważ mamy już 2 referencje inmutable w scopie

    println!("{} {}", r1, r2);

    let r3 = &mut s; // można użyc bo jestesmy już po za scopem uzycia r1 i r2

    r3.pop();

    println!("{}", r3);
}

fn string_slice(some_string: & String) {
    let first_part = &some_string[..2];
    let second_part = &some_string[3..];
    println!("{} {}", first_part, second_part);
    println!("{}", first_word(some_string));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}