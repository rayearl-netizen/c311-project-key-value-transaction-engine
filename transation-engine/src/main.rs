//In rust the args[] stream is instead accessed via a standard library.
use std::env;
use std::fs;

fn main() {
    //Here we access these terminal arguements and store them in a dynamic array -- Vec or vector.
    let args: Vec<String> = env::args().collect();

    //A Rust macro that prints arg with special formatting to support the Vec<String>.
    dbg!(&args);

    //Creating Strings containing all contents of 2 files.
    let contents2 = fs::read_to_string(&args[2])
        .expect("Was not able to find file #2.");
    println!("Text within file number two:\n {contents2}");    
    let contents1 = fs::read_to_string(&args[1])
        .expect("Was not able to find file #1.");
    println!("Text within file number one:\n {contents1}");

    lexical_analyzer("SET".to_string());
}

fn lexical_analyzer(content:String){
    #[derive(Debug)]
    enum Token{
        SET,
        GET,
        DELETE,
        IDENTIFIER(String),
        VALUE(String),
        NEWLINE,
        EOF,
    }
    let iden = Token::IDENTIFIER(content);
    println!("{:?}",iden);
}

