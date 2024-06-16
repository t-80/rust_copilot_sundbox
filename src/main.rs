use calculator::Calculator;

//import the calculator module
mod calculator;

fn main() {
    println!("Hello, world!");
    println!("{}", Calculator::add(10, 7));
}