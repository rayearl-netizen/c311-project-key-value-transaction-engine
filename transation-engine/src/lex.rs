/*
Notes on Syntax Choice.
1. No nested quotes. Example("This is not -> "valid" <- , you see?")
2. No Numerics. Logically doesn't make sense for our purposes, could implement if expanded on.
*/

#[derive(Debug)]
pub enum Token { //Enum that is the basis of tokenization.
    BEGIN,
    END,
    COMMIT,
    ABORT,
    SET,
    GET,
    DEL,
    IDENTIFIER(String),
    VALUE(String),
    SEMICOLON,
    EOF
}
//Function that mutates a vector by pushing the appropriate keyword token compared to the given string.
fn push_keyword(vector: &mut Vec<Token>, slice: &str) {
    match slice { //equivalent to switch statement in Rust.
        "BEGIN" => vector.push(Token::BEGIN),
        "END" => vector.push(Token::END),
        "COMMIT" => vector.push(Token::COMMIT),
        "ABORT" => vector.push(Token::ABORT),
        "SET" => vector.push(Token::SET),
        "GET" => vector.push(Token::GET),
        "DEL" => vector.push(Token::DEL),
        _ => {} //Default is contextually impossible for our uses.
    }
}

fn is_keyword(slice: &str) -> bool {
    matches!(slice, "BEGIN" | "END" | "COMMIT" | "ABORT" | "SET" | "GET" | "DEL") //Using matches! macro for convenience and readability.
}

pub fn lexical_analyzer(file: String) -> Result<Vec<Token>, String> {
    let mut token_vector: Vec<Token> = Vec::new();
    let bytes = file.as_bytes(); //String in Rust do not traverse O(1), thus we convert to bytes.
    let size = bytes.len();
    let mut start: usize;
    let mut walker: usize = 0;
    let mut error: bool = false;


    while walker < size && !error {
        let current_character = bytes[walker] as char;

        if current_character == '#' {
            let mut comment = true;
            while walker < size && comment {
                walker += 1;
                if walker < size && bytes[walker] as char == '~' {
                    walker += 1;
                    if walker < size && bytes[walker] as char == '#'{
                        comment = false;
                    }
                }
            }
            if walker < size  {
                walker += 1;
            }
        }
        else {
            if current_character.is_whitespace() { //Skipping Leading White Space till we reach a significant character
                walker += 1;
            } else if current_character == '"' { //Quote => String => Looser Rules.
                start = walker + 1; //Placing start at beginning of string
                walker += 1; //Beginning search for end of string.

                while walker < size && (bytes[walker] as char) != '"' { //Searching for other "
                    walker += 1;
                }

                if walker >= size { //Checking if other quote found.
                    return Err("Complementing Quote Not Found!".to_string());
                    error = true;
                } else {
                    let slice = (&file[start..walker]).to_string();
                    token_vector.push(Token::VALUE(slice));
                    walker += 1;
                } // !!! We do not support layered quotes. Thus, "Blah "Blah" Blah" is not supported.
            } else if current_character.is_ascii_alphabetic() { //Potential Identifier OR Keyword
                start = walker; //Beginning of word
                walker += 1; //Start search for the end of the word.

                while walker < size && (bytes[walker] as char).is_ascii_alphanumeric() {
                    walker += 1;
                }

                let slice = &file[start..walker]; //Slice of the string.

                if is_keyword(slice) { //if not keyword => unreserved => Identifier.
                    push_keyword(&mut token_vector, slice);
                } else {
                    token_vector.push(Token::IDENTIFIER(slice.to_string()));
                }
            } // !!! Numerics aren't supported in this language due to scope.
            else if current_character == ';' { //consume semicolon
                token_vector.push(Token::SEMICOLON);
                walker += 1;
            } else { //for this language if the above didn't catch => error
                return Err("INVALID LEXEME!!!!".to_string());
                error = true;
            }
        }
    }
    token_vector.push(Token::EOF); //Denoting the EOF
    Ok(token_vector) //returning the created vector of tokens for the parser to observe.
}


