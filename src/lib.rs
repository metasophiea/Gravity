use std::fs::{canonicalize, DirEntry, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

mod utils;
use utils::get_padding;
mod command;
use command::Command;

pub struct Gravity {
    verbose: bool,
    output_file: File,
    output_file_address: PathBuf,
}

impl Gravity {
    fn create_new(verbose:bool, output_file_address:PathBuf) -> Gravity {
        let (output_file, output_file_address) = match File::create(&output_file_address) {
            Ok(file) => (file, canonicalize(&output_file_address).unwrap()),
            Err(err) => panic!("Output file {output_file_address:?} not created - err:{err}"),
        };

        Gravity {
            verbose,
            output_file,
            output_file_address,
        }
    }

    pub fn new(output_file_address:PathBuf) -> Gravity {
        Gravity::create_new(false, output_file_address)
    }
    pub fn new_verbose(output_file_address:PathBuf) -> Gravity {
        Gravity::create_new(true, output_file_address)
    }
}

impl Gravity {
    fn asterisk_include(&mut self, current_working_file_address:Option<&Path>, file_address:&Path, padding:&str) -> bool {
        //separate address into two chunks, on either side of the asterisk
            let mut pre_path = PathBuf::new();
            let mut post_path = PathBuf::new();

            let mut asterisk_encountered = false;
            file_address.components().for_each(|component| {
                if let std::path::Component::Normal(component) = component {
                    if component == std::ffi::OsStr::new("*") {
                        asterisk_encountered = true;
                        return;
                    }
                }

                if asterisk_encountered {
                    post_path.push(component);
                } else {
                    pre_path.push(component);
                }
            });

        //construct every version of the address, and include it
            match std::fs::read_dir(&pre_path) {
                Err(e) => panic!("{e:?}"),
                Ok(paths) => {
                    let mut paths:Vec<DirEntry> = paths.into_iter().filter_map(|path|path.ok()).collect();
                    paths.sort_by(|a, b| a.file_name().partial_cmp(&b.file_name()).unwrap());
    
                    let mut successful_includes = true;
                    for path in paths {
                        let mut path = canonicalize(&path.path()).unwrap();

                        if !path.is_dir() {
                            continue;
                        }

                        path.push(post_path.clone());

                        successful_includes &= self.include(current_working_file_address, &path, padding);
                    }

                    return successful_includes;
                }
            }
    }
    fn include_all_files_of_directory(&mut self, current_working_file_address:Option<&Path>, directory_path:&Path, padding:&str) -> bool {
        match std::fs::read_dir(&directory_path) {
            Err(e) => panic!("{e:?}"),
            Ok(paths) => {
                if self.verbose { println!(" /> including directory: {:?}", canonicalize(&directory_path).unwrap()); }

                let mut paths:Vec<DirEntry> = paths.into_iter().filter_map(|path|path.ok()).collect();
                paths.sort_by(|a, b| a.file_name().partial_cmp(&b.file_name()).unwrap());

                let mut successful_includes = true;
                for path in paths {
                    let path = canonicalize(&path.path()).unwrap();
                    successful_includes &= self.include(current_working_file_address, &path, padding);
                }

                if self.verbose { println!(" </ finished including directory: {:?}", canonicalize(&directory_path).unwrap()); }
                return successful_includes;
            }
        }
    }

    pub fn include(&mut self, current_working_file_address:Option<&Path>, file_address:&Path, padding:&str) -> bool {
        //check for asterisk
            if file_address.components().find(|component| {
                if let std::path::Component::Normal(component) = component {
                    component == &"*"
                } else {
                    false
                }
            }).is_some() {
                return self.asterisk_include(current_working_file_address, file_address, padding);
            }

        //check if file is directory
            match std::fs::metadata(&file_address) {
                Err(_) => {
                    println!("Warning! File or Directory not found: {file_address:?}");
                    return false;
                },
                Ok(data) => {
                    if data.is_dir() {
                        return self.include_all_files_of_directory(current_working_file_address, file_address, padding);
                    }
                },
            }

        
        //check file exists
            if let Err(err_msg) = file_address.try_exists() {
                panic!("could not access file: {file_address:?} - err:{err_msg}")
            }
            let full_file_address = canonicalize(file_address)
                .unwrap_or_else(|err| {
                    panic!("could not find file: {file_address:?} - err: {err}");
                });
        //check that file to be included isn't the output file
            if full_file_address == self.output_file_address {
                println!("Warning! Attempting to include the output file as an input");
                return false;
            }
        //check you're not including yourself
            if let Some(current_working_file_address) = current_working_file_address {
                if current_working_file_address == full_file_address.as_path() {
                    println!("Warning! Attempting to include file into itself");
                    return false;
                }
            }

        //read each line of this file, printing them to output or activating the command
            if let Ok(file) = File::open(file_address) {
                for line in BufReader::new(file).lines() {
                    let line = line.unwrap();
                    if let Some(command) = Command::parse(&line) {
                        self.run_command(&file_address, command, Some(padding));
                    } else {
                        use std::io::prelude::*;
                        match self.output_file.write_all(&format!("{padding}{line}\n").as_bytes()) {
                            Ok(_) => {},
                            Err(err) => panic!("Output file {:?} unwritable - err:{err}", &self.output_file_address),
                        }
                    }
                }
            }

        if self.verbose { println!(" << finished including file: {:?}", &file_address); }

        true
    }
}

impl Gravity {
    fn run_command(&mut self, file_address:&Path, command:Command, command_padding:Option<&str>) {
        if self.verbose { println!(">> running command: {command}"); }

        match command {
            Command::Include { path, mut padding }  => {
                let path = if path.is_absolute() {
                    path
                } else {
                    let mut full_file_address = canonicalize(file_address)
                        .unwrap_or_else(|err| {
                            panic!("could not find file: {file_address:?} - err: {err}");
                        })
                        .parent()
                        .unwrap_or_else(|| {
                            panic!("could not get parent of file: {file_address:?}");
                        })
                        .to_path_buf();
                    full_file_address.push(path);
                    full_file_address
                };

                if let Some(command_padding) = command_padding {
                    padding = format!("{command_padding}{padding}");
                }

                if !self.include(Some(&file_address), &path, &padding) {
                    panic!("Error! include command failed - path:{path:?} padding:\"{padding}\" - Command found in: {file_address:?}");
                }
            },
        }
    }
}