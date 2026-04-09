use crate::parser::Command;
use std::collections::HashMap;

enum RollbackLog{
    OldValue(String),
    Deleted(String),
    Created,
}


pub fn execute_commands(commands:&Vec<Command>, key_value_space:&mut HashMap<String, String>) -> Result<(), String>{
    let mut i:usize = 0;
    let mut rollback_log:HashMap<String, RollbackLog> = HashMap::new();


    while i < commands.len(){
        match &commands[i]{
            Command::BEGIN => println!("BEGIN transaction block"),
            Command::END => {
                rollback_log.clear(); //Automatic Commit when END
                println!("END transaction block. Goodbye!");
            }
            Command::SET(key,value) => {
                let mut oldvalue:String;

                if key_value_space.contains_key(key){ //Checking if we are Changing an Existent Key-Value
                    oldvalue = key_value_space.get(key).expect("Key Not Found!").to_string();
                    if !&rollback_log.contains_key(key){ //Checking if first log.
                        rollback_log.insert(key.to_string(), RollbackLog::OldValue(oldvalue) );
                    }
                }

                key_value_space.insert(key.to_string(), value.to_string()); //Inserting in to Key-Value Space
                println!("SET {key}  as  {value}");
                if !&rollback_log.contains_key(key){ //Check if needs to be logged.
                    rollback_log.insert(key.to_string(), RollbackLog::Created);
                }
            }
            Command::GET(key) => {
                match key_value_space.get(key) {
                    Some(value) => println!("GET returns {}", value),
                    None => println!("Could not find key {}",key)
                }
            }
            Command::DEL(key) => {
                if !&rollback_log.contains_key(key){ //Checking already logged
                    let oldvalue:String = key_value_space.get(key).expect("Not Found").to_string();
                    rollback_log.insert(key.to_string(), RollbackLog::Deleted(oldvalue));
                }
                key_value_space.remove(key);
                println!("DEL {key}")
            }
            Command::COMMIT => {
                rollback_log.clear();
                println!("COMMIT all staged block ")
            }
            Command::ABORT => {
                rollback(&rollback_log,key_value_space);
                rollback_log.clear();
                println!("ABORT staged block")
            }
            _ => {}
        }
        i += 1;
    }
    Ok(())
}

fn rollback(log:&HashMap<String, RollbackLog>, kv_space: &mut HashMap<String, String>) -> (){
    for (key,value) in log{
        match value{
            RollbackLog::OldValue(value) => {
                kv_space.insert(key.to_string(), value.to_string());
            }
            RollbackLog::Deleted(value) => {
                kv_space.insert(key.to_string(), value.to_string());
            }
            RollbackLog::Created => {
                kv_space.remove(key);
            }
            _ => {} //Impossible.
        }
    }
}