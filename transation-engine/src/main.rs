//In rust the args[] stream is instead accessed via a standard library.
use std::env;
use std::fs;

fn main() {
    //Here we access these terminal arguements and store them in a dynamic array -- Vec or vector.
    let args: Vec<String> = env::args().collect();
    //A Rust macro that prints arg with special formatting to support the Vec<String>.
    //Similar to C the first argument will not be a terminal argument but instead the of the
    //binary.
    dbg!(&args);
    if args.len() <= 2{
        eprint!("Not enough arguments!");
    }

    let contents = fs::read_to_string(args[1].clone())
        .expect("Was not able to find this file!");

    println!("Text within file:\n {contents}");

}
