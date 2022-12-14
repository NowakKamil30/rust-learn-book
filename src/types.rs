pub fn run() {
    let x = 1;
    let y = 2.5;
    let x2: i64 = 32432432;
    let y2 = std::i32::MAX;

    let is_active = true;
    let is_greater = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, x2, y2, is_active, is_greater, a1, face));
}