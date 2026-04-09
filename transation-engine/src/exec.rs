use crate::parser::Command;
use std::collections::HashMap;

pub fn execute_commands(commands:Vec<Command>, key_value_space:HashMap<String, String>) -> Result<(), String>{
    let mut i:usize = 0;
    
    while i < commands.len(){
        let mut command = &commands[i];
        match &commands[i]{
            Command::SET(key,value) => {}
            Command::GET(key) => {}
            Command::DEL(key) => {}
            Command::COMMIT => {}
            Command::ABORT => {}
            _ => {}
        }
        i += 1;
    }
    
    Ok(())
}
