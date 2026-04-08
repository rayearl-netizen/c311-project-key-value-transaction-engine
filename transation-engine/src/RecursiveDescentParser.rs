
enum Token {
    Ident
}

enum Command{
    SET,
    GET
}
fn parse_file(tokens:Vec<String>, index : &mut usize) -> Vec<Command>{
    let blank: Vec<Command> = Vec::new();
    //todo!
    return blank //Temp Return!
}

fn parse_stmt(tokens:Vec<String>, index: &mut usize) -> Result<Command, String>{
    //todo!
    Ok(Command::SET)  //Temp Return!
}

fn parse_identifier(tokens:Vec<String>, index: &mut usize) -> Result<String, String> {
    //todo!


    Ok("todo!".to_string())
}

