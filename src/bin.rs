use std::path::{Path, PathBuf};

use gravity::Gravity;

const VERSION:(u8, u8, u8) = (1, 2, 0);

struct CommandInput {
    pub verbosity: u8,
    pub root_file: PathBuf,
    pub output_file: PathBuf,
}
impl CommandInput {
    fn print_help() {
        println!(r#"
Gravity - 2024 Metasophiea - v{}.{}.{}
For compiling together files which use the Gravity in-file commands

Program Arguments
    -r / --root > root file
    -o / --output > output file
    -v / --verbose > verbose mode
    --help > prints the help text

Gravity Commands
    include > a straightforward file-into-file text assembler, eg. {{{{include:aFile.txt}}}}

Examples:
    gravity -r project/mainFile.txt -o build/output.txt
    gravity --root project/mainFile.txt --output build/output.tx --verbose
    gravity --help

Repository
    https://github.com/metasophiea/gravity
"#, VERSION.0, VERSION.1, VERSION.2);
    }
}
impl CommandInput {
    pub fn interpret_args(mut args:Vec<String>) -> CommandInput {
        if args.is_empty() {
            CommandInput::print_help();
            std::process::exit(1);
        }

        let mut verbosity = 0;
        let mut root_file: Option<String> = None;
        let mut output_file: Option<String> = None;

        //collection
            while !args.is_empty() {
                let argument = args.remove(0);

                if argument.find("--") == Some(0) {
                    match &argument[2..] {
                        //verbose
                            "verbose" | "verbose:1" => {
                                verbosity = 1;
                            },
                            "verbose:2" => {
                                verbosity = 2;
                            },
                            "verbose:3" => {
                                verbosity = 3;
                            },
                            "verbose:4" => {
                                verbosity = 4;
                            },

                        //help
                            "help" => {
                                CommandInput::print_help();
                                std::process::exit(1);
                            }

                        //controls
                            "root" => {
                                root_file = Some(args.remove(0));
                            },
                            "output" => {
                                output_file = Some(args.remove(0));
                            },

                        //defaults
                            "" => {
                                println!("Error!: Malformed switch found");
                                std::process::exit(1);
                            }
                            unknown => {
                                println!("Error!: Unknown switch: {}", unknown);
                                std::process::exit(1);
                            },
                    }
                } else if argument.find("-") == Some(0) {
                    match &argument.chars().skip(1).next() {
                        Some('v') => {
                            verbosity = 1;
                        },
                        Some('r') => {
                            root_file = Some(args.remove(0));
                        },
                        Some('o') => {
                            output_file = Some(args.remove(0));
                        },
                        None => {
                            println!("Error!: Malformed switch found");
                            std::process::exit(1);
                        }
                        Some(unknown) => {
                            println!("Error!: Unknown switch: {}", unknown);
                            std::process::exit(1);
                        },
                    }
                }
            }

        //checking
            if root_file == None {
                println!("No root file found");
                std::process::exit(1);
            }
            if output_file == None {
                println!("No output file found");
                std::process::exit(1);
            }

        //reformat
            let root_file = PathBuf::from(root_file.unwrap()).canonicalize().unwrap();
            let output_file = PathBuf::from(output_file.unwrap()).canonicalize().unwrap();

        //check root file exists
            if !root_file.exists() {
                println!("Root file address does not exist - {root_file:?}");
                std::process::exit(1);
            }

        //check output file can exist
            if !output_file.parent().unwrap().exists() {
                println!("Output file address can not exist - {output_file:?}");
                std::process::exit(1);
            }

        //return
            CommandInput {
                verbosity,
                root_file,
                output_file
            }
    }
}   

fn main() {
    //interpret input
        let command_input = CommandInput::interpret_args(std::env::args().skip(1).collect());

    //initialize gravity
        let mut obj = if command_input.verbosity > 0 {
            Gravity::new(command_input.output_file)
        } else {
            Gravity::new_verbose(command_input.output_file)
        };
        
    //include root file
        obj.include(
            Some(&command_input.root_file),
            Path::new(""),
            ""
        );
}