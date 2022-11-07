mod command;
use command::Command;

mod library;

pub struct Gravity {
    verbose: bool,
    output_file: std::fs::File,
    output_file_address: String,
}
impl Gravity {
    pub fn new(verbose: bool, output_file_address: String) -> Gravity {
        let (output_file, output_file_address) = match std::fs::File::create(&output_file_address) {
            Ok(file) => {
                (file, library::get_full_address_of_file(&output_file_address).unwrap())
            },
            Err(err) => {
                println!("Output file {} not created", &output_file_address);
                println!("{}",err);
                std::process::exit(1);
            },
        };

        Gravity {
            verbose,
            output_file,
            output_file_address,
        }
    }

    fn run_command(&mut self, command: Command, full_file_address: &str) {
        let full_file_address = match library::get_full_address_of_file(full_file_address) {
            Ok(a) => a,
            Err(_) => {
                println!("cound not find file: {}", &full_file_address);
                std::process::exit(1);
            },
        };

        match command.name.as_str() {
            "include" => {
                let sucessful_include_execution = match command.arguments[0].chars().next() {
                    Some('/') => self.include( &command.arguments[0], &command.hidden_arguments[0], &full_file_address ),
                    _ => {
                        let full_file_directory = library::get_full_directory_of_file(&full_file_address);
                        self.include( &format!("{}/{}", full_file_directory, command.arguments[0]), &command.hidden_arguments[0], &full_file_address )
                    },
                };

                if !sucessful_include_execution {
                    println!("");
                    println!("Error! command failed: {}",command);
                    println!("| Command found in: {}",full_file_address);
                    std::process::exit(1);
                }
            },
            _ => {
                println!("Error! unknown command: {}", command);
                std::process::exit(1);
            },
        }
    }
    
    pub fn include(&mut self, file_address: &str, padding: &str, current_working_file_address: &str) -> bool {
        fn recursive_include(this: &mut Gravity, paths: std::fs::ReadDir, post_address_string: &str, padding: &str, current_working_file_address: &str) -> bool {
            let mut sucessful_includes = true;
            for path in paths {
                let file = path.unwrap().path();
                let file_this_time = format!("{}{}", &file.to_str().unwrap(), &post_address_string);
                let file_this_time = match library::get_full_address_of_file(&file_this_time) {
                    Ok(address) => address,
                    Err(_) => {
                        println!("Error! Could not find file: {}", file_this_time);
                        return false;
                    },
                };
                
                if &file_this_time == &current_working_file_address || &file_this_time == &this.output_file_address {
                    continue;
                }
                sucessful_includes &= this.include(&file_this_time, &padding, &current_working_file_address);
            }
            sucessful_includes
        }

        //check for asterisk
            match file_address.find("*") {
                Some(asterisk_index) => {
                    let file_directory = library::get_full_directory_of_file(&file_address[..asterisk_index]);
                    match std::fs::read_dir(&file_directory) {
                        Err(e) => {
                            println!("{:?}",e);
                            std::process::exit(1);
                        },
                        Ok(paths) => {
                            return recursive_include(
                                self,
                                paths,
                                &file_address[asterisk_index+1..],
                                &padding,
                                &current_working_file_address
                            );
                        }
                    };
                },
                None => {},
            }

        //check if file is directory
            match std::fs::metadata(&file_address) {
                Err(_) => {
                    println!("Error! File or Directory not found: {}", &file_address);
                    return false;
                },
                Ok(data) => {
                    if data.is_dir() {
                        match std::fs::read_dir(&file_address) {
                            Err(e) => {
                                println!("{:?}",e);
                                std::process::exit(1);
                            },
                            Ok(paths) => {
                                if self.verbose { println!(" /> including directory: {}", library::get_full_address_of_file(&file_address).unwrap()); }
        
                                let sucessful_includes = recursive_include(
                                    self,
                                    paths,
                                    "",
                                    &padding,
                                    &current_working_file_address
                                );

                                if self.verbose { println!(" </ finished including directory: {}", library::get_full_address_of_file(&file_address).unwrap()); }
                                return sucessful_includes;
                            }
                        };
                    }
                },
            }

        //read each line of this file, printing them to output or activating the command
            if let Ok(lines) = library::read_lines(&file_address) {
                for (index, line) in lines.enumerate() {
                    let line = line.unwrap();
                    if let Some(mut command) = library::get_command_from_string(&line) {
                        if self.verbose { println!("{} | {} | command found: \"{}\"", &file_address, index, line); }
                        command.hidden_arguments[0] = format!("{}{}", &padding, command.hidden_arguments[0]);
                        self.run_command(command,&file_address);
                    } else {
                        use std::io::prelude::*;
                        match self.output_file.write_all(&format!("{}{}\n",padding,line).as_bytes()) {
                            Ok(_) => {},
                            Err(err) => {
                                println!("Output file {} unwritable", &self.output_file_address);
                                println!("{}",err);
                                std::process::exit(1);
                            }
                        }
                    }
                }
            }

        if self.verbose { println!(" << finished including file: {}", library::get_full_address_of_file(&file_address).unwrap()); }

        true
    }
}