use std::collections::HashSet;
use crate::parser::Command;

struct ValidatorState{
    identifiers:HashSet<String>,
    saw_begin:bool,
    saw_end:bool,
    active_transaction:bool
}

//"Silent Bounce Approach", either the validator announces an error or we proceed.
pub fn validator(command_stream: Vec<Command>) -> Result<(), String>{
    let mut i = 0;
    let mut state:ValidatorState = ValidatorState{
        identifiers: HashSet::new(),
        saw_begin: false,
        saw_end: false,
        active_transaction:false
    };

    while i < command_stream.len(){
        let mut command = &command_stream[i];
        match &command_stream[i]{
            Command::SET(key,_) => validate_set(key.to_string() ,&mut state)?,
            Command::GET(_) => validate_identifier_ref(command, &mut state)?,
            Command::DEL(_) => validate_identifier_ref(command,&mut state )?,
            Command::BEGIN => validate_begin(&mut state)?,
            Command::END => validate_end(&mut state)?,
            Command::COMMIT => validate_basics(&mut state)?,
            Command::ABORT => validate_basics(&mut state)?,
            _ => {}
        }
        i += 1;
    }
    Ok(())
}
//GET and DEL commands
fn validate_identifier_ref(command: &Command,  state:&mut ValidatorState) -> Result<(), String>{
    let key = match command {
        Command::GET(identifier) => identifier,
        Command::DEL(identifier) => identifier,
        _ => return Err("Expected Command with Identifier Reference".to_string()),
    };
    if state.active_transaction != true {
        return Err("GET/DEL must be within BEGIN-END block".to_string());
    }
    if !state.identifiers.contains(key){
        return  Err("Key not found!".to_string());
    }
    Ok(())
}

fn validate_begin(state: &mut ValidatorState) -> Result<(), String> {
    if state.saw_begin {
        return Err("Already within Transaction Block".to_string());
    }
    if state.saw_end {
        return Err("Cannot BEGIN after END".to_string());
    }

    state.saw_begin = true;
    state.active_transaction = true;
    Ok(())
}

fn validate_end(state: &mut ValidatorState) -> Result<(), String>{
    if !state.saw_begin{
        return Err("Not Within a Transaction Block. Cannot End".to_string());
    }
    state.active_transaction = true;
    state.saw_end = true;
    Ok(())
}


fn validate_set(key:String, state: &mut ValidatorState) -> Result<(), String> {
    if !state.active_transaction {
        return Err("SET must be within BEGIN-END block".to_string());
    }

    state.identifiers.insert(key.clone());
    Ok(())
}


fn validate_basics(state: &mut ValidatorState) -> Result<(), String> {
    if !state.active_transaction {
        return Err("COMMIT/ABORT must be within BEGIN-END block".to_string());
    }

    Ok(())
}
