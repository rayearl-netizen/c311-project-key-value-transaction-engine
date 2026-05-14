use std::collections::HashSet;
use crate::parser::Command;

struct ValidatorState{
    identifiers:HashSet<String>,
    saw_begin:bool,
    saw_end:bool,
    active_transaction:bool
}

//"Silent Bounce Approach", either the validator announces an error or we proceed.
pub fn validator(command_stream: &Vec<Command>) -> Result<(), String>{
    let mut i = 0;
    let mut state:ValidatorState = ValidatorState{
        identifiers: HashSet::new(),
        saw_begin: false,
        saw_end: false,
        active_transaction:false
    };
    //iterating through command stream sequentially.
    while i < command_stream.len(){
        let command = &command_stream[i];
        //matching each unique command to its own helper function for validation.
        match &command_stream[i]{
            Command::SET(key,_) => validate_set(key.to_string() ,&mut state)?,
            Command::GET(_) => validate_get(command, &mut state)?,
            Command::DEL(_) => validate_del(command,&mut state )?,
            Command::BEGIN => validate_begin(&mut state)?,
            Command::END => validate_end(&mut state)?,
            Command::COMMIT => validate_basics(&mut state)?,
            Command::ABORT => validate_basics(&mut state)?
        }
        i += 1;
    }
    //If all goes well, we silently exit.
    Ok(())
}
//GET and DEL commands
fn validate_get(command: &Command,  state:&mut ValidatorState) -> Result<(), String>{
    //need match for extracting while accounting for error.
    let key = match command {
        Command::GET(identifier) => identifier,
        _ => return Err("Expected Command with Identifier Reference".to_string()),
    };
    //checking if within BEGIN-END block and checking if relevant identifier exists within current scope.
    if state.active_transaction == false {
        return Err("GET must be within BEGIN-END block".to_string());
    }
    if !state.identifiers.contains(key){
        return  Err("Key not found!".to_string());
    }
    //silent exit if all goes well.
    Ok(())
}


//almost identical to get except...
fn validate_del(command: &Command,  state:&mut ValidatorState) -> Result<(), String>{
    if state.active_transaction != true {
        return Err("DEL must be within BEGIN-END block".to_string());
    }

    let key = match command {
        Command::DEL(identifier) => identifier,
        _ => return Err("Expected Command with Identifier Reference".to_string()),
    };

    if !state.identifiers.contains(key){
        return  Err("Key not found!".to_string());
    }
    //we must remove identifier from scope before exiting.
    state.identifiers.remove(key);

    Ok(())
}

fn validate_begin(state: &mut ValidatorState) -> Result<(), String> {
    if state.saw_begin {
        return Err("Already within Transaction Block".to_string());
    }
    if state.saw_end {
        return Err("Cannot BEGIN after END".to_string());
    }
    //Contextually checks for double begin.
    state.saw_begin = true;
    state.active_transaction = true;
    Ok(())
}

fn validate_end(state: &mut ValidatorState) -> Result<(), String>{
    if !state.saw_begin{
        return Err("Not Within a Transaction Block. Cannot End".to_string());
    }
    //contextually checks for corresponding begin.
    state.active_transaction = false;
    state.saw_end = true;
    Ok(())
}


fn validate_set(key:String, state: &mut ValidatorState) -> Result<(), String> {
    if !state.active_transaction {
        return Err("SET must be within BEGIN-END block".to_string());
    }
    //simply checks if within active block and adds appropriate identifier.
    state.identifiers.insert(key.clone());
    Ok(())
}

//Commit and Abort both don't mutate in a manner that warrants, a rollback and only require active block.
fn validate_basics(state: &mut ValidatorState) -> Result<(), String> {
    if !state.active_transaction {
        return Err("COMMIT/ABORT must be within BEGIN-END block".to_string());
    }

    Ok(())
}
