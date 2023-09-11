use std::path::PathBuf;

use super::get_padding;

pub enum Command {
    Include { path:PathBuf, padding:String }
}

impl Command {
    pub fn parse(potential_command:&str) -> Option<Command> {
        if let (Some(start_index), Some(colon_index), Some(end_index)) = (potential_command.find("{{"), potential_command.find(":"), potential_command.find("}}")) {            
            if let Some(comment_index) = potential_command.find("//") {
                if comment_index < start_index {
                    return None
                }
            }
            
            let command = &potential_command[start_index+2..colon_index];
            let file = &potential_command[colon_index+1..end_index];
            let padding = get_padding(&potential_command);
    
            match command {
                "include" => {
                    Some(
                        Command::Include { path:PathBuf::from(file), padding }
                    )
                },
                _ => {
                    None
                },
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
         match self {
            Command::Include{path, padding} => write!(f, "Include {{ path:{path:?}, padding:\"{padding}\"}}")
        }
    }
}