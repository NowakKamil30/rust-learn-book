pub(crate) fn run (number_of_elements: i32) {
    if number_of_elements <= 0 {
        println!("number of elements must be positive");
        return;
    }
    let mut x: u128 = 0;
    let mut y: u128 = 1;
    println!("item number: {} value dec: {} value bin {:b}", 1, 0, 0);
    if number_of_elements == 1 {
        return;
    }
    println!("item number: {} value: {} value bin {:b}", 2, 1, 1);
    for i in 2..number_of_elements {
        let temp = y;
        y += x;
        x = temp;
        println!("item number: {} value: {} value bin {:b}", i + 1, y, y);
    }
}