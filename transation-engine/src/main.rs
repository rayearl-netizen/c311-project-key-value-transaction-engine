mod lex;
mod parser;
mod validator;
pub mod exec;

//In rust the args[] stream is instead accessed via a standard library.
use std::env;
use std::fs;
use std::collections::HashMap;


fn main() -> Result<(),String>{
    //Here we access these terminal arguements and store them in a dynamic array -- Vec or vector.
    let args: Vec<String> = env::args().collect();

    //A Rust macro that prints arg with special formatting to support the Vec<String>.
    //dbg!(&args);

    let mut key_value_space: HashMap<String,String> = HashMap::new();



    for i in 1..args.len() {
        let contents = fs::read_to_string(&args[i]).expect("Not able to find file"); //Large String for Processing File 1.
        println!("\nText within file number {i}: \n{contents}");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        println!("\nBeginning Compilation of Text File {i}");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        let tokenized = lex::lexical_analyzer(contents)?;
        println!("\nLexical Analysis");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        for i in 0..tokenized.len() {
            println!("{:?}", tokenized[i]);
        }
        println!("Lexing Complete!");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        println!("\nParsing");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        let parsed = parser::parse_tokens(tokenized).expect("Parsing Error!");
        for i in 0..parsed.len() {
            println!("{:?}", parsed[i]);
        }
        println!("Parsing Complete!");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        println!("\nBeginning Validation!");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        validator::validator(&parsed)?;
        println!("Validation Complete!");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");


        println!("\nStaging Execution!\n");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        exec::execute_commands(&parsed, &mut key_value_space)?;
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        println!("\nCompilation and Execution of File {i} Complete!");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    }

    return Ok(())
}

