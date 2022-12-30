use std::fs::{DirEntry, File};

mod command;
use command::Command;

mod library;

pub struct Gravity {
    verbose: bool,
    output_file: File,
    output_file_address: String,
}

impl Gravity {
    fn create_new(verbose:bool, output_file_address:String) -> Gravity {
        let (output_file, output_file_address) = match File::create(&output_file_address) {
            Ok(file) => (file, library::get_full_address_of_file(&output_file_address).unwrap()),
            Err(err) => panic!("Output file {output_file_address} not created - err:{err}"),
        };

        Gravity {
            verbose,
            output_file,
            output_file_address,
        }
    }

    pub fn new(output_file_address:String) -> Gravity {
        Gravity::create_new(false, output_file_address)
    }
    pub fn new_verbose(output_file_address:String) -> Gravity {
        Gravity::create_new(true, output_file_address)
    }
}

impl Gravity {
    pub fn include(&mut self, file_address:&str, padding:&str, current_working_file_address:&str) -> bool {
        fn recursive_include(this:&mut Gravity, paths:std::fs::ReadDir, post_address_string:&str, padding:&str, current_working_file_address:&str) -> bool {
            let mut successful_includes = true;

            let mut paths:Vec<DirEntry> = paths.into_iter().filter_map(|path|path.ok()).collect();
            paths.sort_by(|a, b| a.file_name().partial_cmp(&b.file_name()).unwrap());

            for path in paths {
                let file = path.path();
                let file_this_time = format!("{}{post_address_string}", &file.to_str().unwrap());
                let file_this_time = match library::get_full_address_of_file(&file_this_time) {
                    Ok(address) => address,
                    Err(_) => {
                        println!("Warning! Could not find file: {file_this_time}");
                        return false;
                    },
                };
                
                if &file_this_time == &current_working_file_address || &file_this_time == &this.output_file_address {
                    continue;
                }
                successful_includes &= this.include(&file_this_time, &padding, &current_working_file_address);
            }

            successful_includes
        }

        //check for asterisk
            match file_address.find("*") {
                Some(asterisk_index) => {
                    let file_directory = library::get_full_directory_of_file(&file_address[..asterisk_index]);
                    match std::fs::read_dir(&file_directory) {
                        Err(e) => panic!("{e:?}"),
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
                    println!("Warning! File or Directory not found: {file_address}");
                    return false;
                },
                Ok(data) => {
                    if data.is_dir() {
                        match std::fs::read_dir(&file_address) {
                            Err(e) => panic!("{e:?}"),
                            Ok(paths) => {
                                if self.verbose { println!(" /> including directory: {}", library::get_full_address_of_file(&file_address).unwrap()); }
        
                                let successful_includes = recursive_include(
                                    self,
                                    paths,
                                    "",
                                    &padding,
                                    &current_working_file_address
                                );

                                if self.verbose { println!(" </ finished including directory: {}", library::get_full_address_of_file(&file_address).unwrap()); }
                                return successful_includes;
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
                        if self.verbose { println!("{file_address} | {index} | command found: \"{line}\""); }
                        command.hidden_arguments[0] = format!("{padding}{}", command.hidden_arguments[0]);
                        self.run_command(command,&file_address);
                    } else {
                        use std::io::prelude::*;
                        match self.output_file.write_all(&format!("{padding}{line}\n").as_bytes()) {
                            Ok(_) => {},
                            Err(err) => panic!("Output file {} unwritable - err:{err}", &self.output_file_address),
                        }
                    }
                }
            }

        if self.verbose { println!(" << finished including file: {}", library::get_full_address_of_file(&file_address).unwrap()); }

        true
    }
}

impl Gravity {
    fn run_command(&mut self, command:Command, full_file_address:&str) {
        let full_file_address = match library::get_full_address_of_file(full_file_address) {
            Ok(a) => a,
            Err(_) => panic!("could not find file: {full_file_address}"),
        };

        match command.name.as_str() {
            "include" => {
                let successful_include_execution = match command.arguments[0].chars().next() {
                    Some('/') => self.include( &command.arguments[0], &command.hidden_arguments[0], &full_file_address ),
                    _ => {
                        let full_file_directory = library::get_full_directory_of_file(&full_file_address);
                        self.include( &format!("{full_file_directory}/{}", command.arguments[0]), &command.hidden_arguments[0], &full_file_address )
                    },
                };

                if !successful_include_execution {
                    panic!("Error! command failed: {command} - Command found in: {full_file_address}");
                }
            },
            _ => {
                panic!("Error! unknown command: {command}");
            },
        }
    }
}