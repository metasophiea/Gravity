use std::path::PathBuf;

fn get_leading_whitespace(string:&str) -> String {
    let mut count = 0;
    for c in string.chars() {
        if c != ' ' {
            break
        }
        count += 1;
    }
    " ".repeat(count)
}

pub enum Command {
    Include(PathBuf, String)
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
            let padding = get_leading_whitespace(&potential_command);
    
            match command {
                "include" => {
                    Some(
                        Command::Include(PathBuf::from(file), padding)
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
            Command::Include(path, padding) => write!(f, "Include({path:?}, \"{padding}\")")
        }
    }
}