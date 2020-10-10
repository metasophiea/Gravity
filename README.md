<p align="center">
    <img width="75%" src="https://raw.githubusercontent.com/metasophiea/Gravity/main/logo.png">
</p>

For compiling together files which use the Gravity in-file commands

## Program Arguments

    -r / --root > root file
    -o / --output > output file
    -v / --verbose > verbose mode
    --help > prints the help text

## Gravity Commands

    include > a straightforward file-into-file text assembler, eg. {{include:aFile.txt}}

## Examples

    gravity -r project/mainFile.txt -o build/output.txt
    gravity --root project/mainFile.txt --output build/output.tx --verbose
    gravity --help
