mod gravity;

// gravity -r test/test_1.txt -o test/output.txt -v
// gravity --root test/test_1.txt --output test/output.txt --verbose
// gravity --help

fn main() {
    let version = "v1";

    let mut args = std::env::args().skip(1);

    let mut verbose = false;
    let mut root_file: Option<String> = None;
    let mut output_file: Option<String> = None;

    while let Some(argument) = args.next() {
        if argument.find("--") == Some(0) {
            match &argument[2..] {
                "root" => {
                    root_file = Some(args.next().unwrap().clone());
                },
                "output" => {
                    output_file = Some(args.next().unwrap().clone());
                },
                "verbose" => {
                    verbose = true;
                },
                "help" => {
                    println!("
Gravity - 2020 Metasophiea - {}
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
                    ", version);
                    std::process::exit(1);
                },
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
                Some('r') => {
                    root_file = Some(args.next().unwrap().clone());
                },
                Some('o') => {
                    output_file = Some(args.next().unwrap().clone());
                },
                Some('v') => {
                    verbose = true;
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

    if root_file == None {
        println!("No root file found");
        std::process::exit(1);
    }
    if output_file == None {
        println!("No output file found");
        std::process::exit(1);
    }

    let mut obj = gravity::Obj::new(
        verbose,
        output_file.unwrap().to_string()
    );
    obj.include( &root_file.unwrap(), "", "" );
}