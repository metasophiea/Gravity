use std::io::BufRead;

use crate::gravity::command::Command;

pub fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where P: AsRef<std::path::Path>, {
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
pub fn get_command_from_string(potential_command: &str) -> Option<Command> {
    if let (Some(start_index), Some(colon_index), Some(end_index)) = (potential_command.find("{{"), potential_command.find(":"), potential_command.find("}}")) {            
        if let Some(comment_index) = potential_command.find("//") {
            if comment_index < start_index {
                return None
            }
        }
        
        let command = &potential_command[start_index+2..colon_index];
        let file = &potential_command[colon_index+1..end_index];
        let padding = get_leading_whitespace(&potential_command);

        Some(Command {
            name: command.to_string(),
            arguments: vec![
                file.to_string(),
            ],
            hidden_arguments: vec![
                padding
            ],
        })
    } else {
        None
    }
}
pub fn get_full_address_of_file(address: &str) -> Result<String,String> {
    match std::fs::canonicalize(std::path::PathBuf::from(&address)) {
        Ok(file) => {
            Ok(file.to_str().unwrap().to_string())
        },
        Err(_) => {
            return Err("file not found".to_string());
        },
    }
}
pub fn get_full_directory_of_file(file_address: &str) -> String {
    get_full_address_of_file(&file_address[..file_address.rfind("/").unwrap()]).unwrap()
}
pub fn get_leading_whitespace(string: &str) -> String {
    let mut count = 0;
    for c in string.chars() {
        if c != ' ' {
            break
        }
        count += 1;
    }
    " ".repeat(count)
}