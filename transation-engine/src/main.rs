mod lex;
mod parser;
mod validator;
pub mod exec;

//In rust the args[] stream is instead accessed via a standard library.
use std::env;
use std::fs;
use std::collections::HashMap;


fn main() {
    //Here we access these terminal arguements and store them in a dynamic array -- Vec or vector.
    let args: Vec<String> = env::args().collect();

    //A Rust macro that prints arg with special formatting to support the Vec<String>.
    dbg!(&args);

    let mut key_value_space: HashMap<String,String> = HashMap::new();
    
    let contents1 = fs::read_to_string(&args[1]).expect("Not able to find file"); //Large String for Processing File 1.
    println!("Text within file number one: \n{contents1}");
    let contents2 = fs::read_to_string(&args[2]).expect("Not able to find file"); //Large String for Processing File 2.
    println!("Text within file number one: \n{contents2}");

    let tokenized = lex::lexical_analyzer(contents1);
    println!("Lexical Analysis");
    for i in 0..tokenized.len() {
        println!("{:?}", tokenized[i]);
    }

    let parsed = parser::parse_tokens(tokenized).expect("Parsing Error!");

    println!("\nParsing");
    for i in 0.. parsed.len(){
        println!("{:?}", parsed[i]);
    }

    validator::validator(&parsed);
    println!("Validation Complete!");
    
    exec::execute_commands(parsed, key_value_space);
    
        
}

