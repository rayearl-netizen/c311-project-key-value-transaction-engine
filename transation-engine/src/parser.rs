use crate::lex::Token;

#[derive(Debug)]
pub enum Command {
    SET(String, String), //Should be SET IDENTIFIER VALUE SEMICOLON
    GET(String), //GET IDENTIFIER SEMICOLON
    DEL(String), //DEL IDENTIFIER SEMICOLON
    BEGIN, //BEGIN SEMICOLON
    END, //END SEMICOLON
    COMMIT, //COMMIT SEMICOLON
    ABORT, // ABORT SEMICOLON
}
//Result is beneficial for returning a datatype XOR allowing for an error passed back instead.
pub fn parse_tokens(token_stream: Vec<Token>) -> Result<Vec<Command>, String> {
    let mut command_stream = Vec::new();
    let mut i = 0;

    //Traversing through token stream.
    while i < token_stream.len() {
        let command = match &token_stream[i] {
            Token::BEGIN => parse_begin(&token_stream, &mut i)?,
            Token::END => parse_end(&token_stream, &mut i)?,
            Token::COMMIT => parse_commit(&token_stream, &mut i)?,
            Token::ABORT => parse_abort(&token_stream, &mut i)?,
            Token::SET => parse_set(&token_stream, &mut i)?,
            Token::GET => parse_get(&token_stream, &mut i)?,
            Token::DEL => parse_del(&token_stream, &mut i)?,
            Token::IDENTIFIER(name) => {
                return Err(format!("Unexpected identifier '{}' at top level", name)); //Err necessary to return on Result and error.
            }
            Token::VALUE(value) => {
                return Err(format!("Unexpected value '{}' at top level", value));
            }
            Token::EOF => break, //Leave loop once EOF is encountered
            _ => {return Err(format!("Unexpected token"));}
        };
        command_stream.push(command);
    }
    Ok(command_stream)
}


//Each of the following parse a unique keyword consuming its arguments and detecting incorrect arguments,
//then returning a COMMAND enum containing all necessary info for validation and execution.
fn parse_begin(tokens: &Vec<Token>, i: &mut usize) -> Result<Command, String>{
    //passing primitive by reference for mutation outside of function.
    *i+=1;
    // tokens.get(*i) returns Option<&Token>.
    // Some() means a &token exists here and matches the pattern. Necessary to compile!
    match tokens.get(*i) {
        Some(Token::SEMICOLON) => *i+=1,
        _ => return Err("Expected SEMICOLON after BEGIN".to_string()),
    };

    Ok(Command::BEGIN)
}

fn parse_end(tokens: &Vec<Token>, i: &mut usize) -> Result<Command, String>{
    *i+=1;
    //Simply consumes semicolon and adds END command.
    match tokens.get(*i) {
        Some(Token::SEMICOLON) => *i+=1,
        _ => return Err("Expected SEMICOLON after END".to_string())
    };

    Ok(Command::END)
}

fn parse_commit(tokens: &Vec<Token>, i: &mut usize) -> Result<Command, String>{
    *i+=1;

    //Simply consumes semicolon and adds COMMIT commmand.
    match tokens.get(*i) {
        Some(Token::SEMICOLON) => *i+=1,
        _ => return Err("Expected SEMICOLON after COMMIT".to_string())
    };

    Ok(Command::COMMIT)
}

fn parse_get(tokens: &Vec<Token>, i: &mut usize) -> Result<Command, String>{
    *i+=1;
    
    //checks for identifier token.
    let key = match tokens.get(*i) {
        Some(Token::IDENTIFIER(name)) => name.clone(),
        _ => return Err("Expected IDENTIFIER after GET".to_string())
    };
    
    *i+=1;
    //checks for semicolon token.
    match tokens.get(*i) {
        Some(Token::SEMICOLON) => *i+=1,
        _ => return Err("Expected SEMICOLON after IDENTIFIER".to_string())
    };

    //returns GET command enum with referencing key within.
    Ok(Command::GET(key))
}
fn parse_del(tokens: &Vec<Token>, i: &mut usize) -> Result<Command, String>{
    *i+=1;
    
    //Checks for identifier
    let key = match tokens.get(*i) {
        Some(Token::IDENTIFIER(name)) => name.clone(),
        _ => return Err("Expected IDENTIFIER after GET".to_string())
    };

    *i+=1;

    //checks for semicolon.
    match tokens.get(*i) {
        Some(Token::SEMICOLON) => *i+=1,
        _ => return Err("Expected SEMICOLON after DEL".to_string())
    };
    //returns COMMAND enum with referencing key.
    Ok(Command::DEL(key))
}




fn parse_abort(tokens: &Vec<Token>, i: &mut usize) -> Result<Command, String>{
    *i+=1;

    //simply consumes semicolon.
    match tokens.get(*i) {
        Some(Token::SEMICOLON) => *i+=1,
        _ => return Err("Expected SEMICOLON after ABORT".to_string())
    };

    Ok(Command::ABORT)
}

fn parse_set(tokens: &Vec<Token>, i: &mut usize) -> Result<Command, String> {
    //i is a mutable pass by reference, this updates the state within the parse_tokens function!
    *i += 1;

    //checks for identifier token
    let key = match tokens.get(*i) {
        //Some() function checks existence. tokens.get(*i) returns True, so we must follow suit
        Some(Token::IDENTIFIER(name)) => name.clone(),
        _ => return Err("Expected IDENTIFIER after SET".to_string()),
    };

    *i += 1;

    //then checks for value token.
    let value = match tokens.get(*i) {
        Some(Token::VALUE(val)) => val.clone(),
        _ => return Err("Expected VALUE after IDENTIFIER in SET".to_string()),
    };

    *i += 1;

    //then consumes semicolon token
    match tokens.get(*i) {
        Some(Token::SEMICOLON) => *i+=1,
        _ => return Err("Expected SEMICOLON after VALUE in SET".to_string()),
    };

    //returns command enum with key and value within
    Ok(Command::SET(key, value))
}



