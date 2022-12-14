mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vector;
mod conditional;
mod loops;
mod function;
mod pointer_and_ref;
mod structs;
mod enums;
mod cli;
mod fib;

fn main() {
    println!("Hello, world!");
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vector::run();
    conditional::run();
    loops::run();
    function::run();
    pointer_and_ref::run();
    structs::run();
    enums::run();
    cli::run();
    fib::run(100);
}
