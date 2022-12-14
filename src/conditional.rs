pub fn run() {
    let age = 18;
    let check_id = true;
    if age >= 21 && check_id {
        println!("yeah");
    } else if !check_id {
        println!("yeah!!!");
    } else {
        println!("Nope");
    }

    let is_of_age = if age >= 21 {true} else {false};
}