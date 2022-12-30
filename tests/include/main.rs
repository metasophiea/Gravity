use std::fs::{remove_file, File};
use std::io::{BufReader, Read};
use std::path::Path;

use gravity::Gravity;

fn check_file_equality(file_address_a:&str, file_address_b:&str) -> Result<bool, &'static str> {
    if let Result::Ok(file_a) = File::open(file_address_a) {
        let mut reader1 = BufReader::new(file_a);
        if let Result::Ok(file_b) = File::open(file_address_b) {
            let mut reader2 = BufReader::new(file_b);
            let mut buf1 = [0; 10000];
            let mut buf2 = [0; 10000];
            loop {
                if let Result::Ok(n1) = reader1.read(&mut buf1) {
                    if n1 > 0 {
                        if let Result::Ok(n2) = reader2.read(&mut buf2) {
                            if n1 == n2 {
                                if buf1 == buf2 {
                                    continue;
                                }
                            }
                            return Ok(false);
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            return Ok(true);
        } else {
            return Err("file b not found")
        }
    } else {
        return Err("file a not found")
    }
}

#[test]
fn include_1() {
    const OUTPUT_FILE_NAME:&str = "tests/include/test_structure_1/output.txt";

    let mut gravity = Gravity::new_verbose(OUTPUT_FILE_NAME.into());
    gravity.include(None, Path::new("tests/include/test_structure_1/input.txt"), "");

    match check_file_equality(
        OUTPUT_FILE_NAME,
        "tests/include/test_structure_1/correct_output.txt"
    ) {
        Ok(result) => assert!(result),
        Err(err_msg) => panic!("{err_msg}"),
    }

    remove_file(OUTPUT_FILE_NAME).unwrap();
}

#[test]
fn include_2() {
    const OUTPUT_FILE_NAME:&str = "tests/include/test_structure_2/output.txt";

    let mut gravity = Gravity::new_verbose(OUTPUT_FILE_NAME.into());
    gravity.include(None, Path::new("tests/include/test_structure_2/input.txt"), "");

    match check_file_equality(
        OUTPUT_FILE_NAME,
        "tests/include/test_structure_2/correct_output.txt"
    ) {
        Ok(result) => assert!(result),
        Err(err_msg) => panic!("{err_msg}"),
    }

    remove_file(OUTPUT_FILE_NAME).unwrap();
}

#[test]
fn include_3() {
    const OUTPUT_FILE_NAME:&str = "tests/include/test_structure_3/output.txt";

    let mut gravity = Gravity::new_verbose(OUTPUT_FILE_NAME.into());
    gravity.include(None, Path::new("tests/include/test_structure_3/input.txt"), "");

    match check_file_equality(
        OUTPUT_FILE_NAME,
        "tests/include/test_structure_3/correct_output.txt"
    ) {
        Ok(result) => assert!(result),
        Err(err_msg) => panic!("{err_msg}"),
    }

    remove_file(OUTPUT_FILE_NAME).unwrap();
}

#[test]
fn include_4() {
    const OUTPUT_FILE_NAME:&str = "tests/include/test_structure_4/output.txt";

    let mut gravity = Gravity::new_verbose(OUTPUT_FILE_NAME.into());
    gravity.include(None, Path::new("tests/include/test_structure_4/input.txt"), "");

    match check_file_equality(
        OUTPUT_FILE_NAME,
        "tests/include/test_structure_4/correct_output.txt"
    ) {
        Ok(result) => assert!(result),
        Err(err_msg) => panic!("{err_msg}"),
    }

    remove_file(OUTPUT_FILE_NAME).unwrap();
}

#[test]
fn include_5() {
    const OUTPUT_FILE_NAME:&str = "tests/include/test_structure_5/output.txt";

    let mut gravity = Gravity::new_verbose(OUTPUT_FILE_NAME.into());
    gravity.include(None, Path::new("tests/include/test_structure_5/input.txt"), "");

    match check_file_equality(
        OUTPUT_FILE_NAME,
        "tests/include/test_structure_5/correct_output.txt"
    ) {
        Ok(result) => assert!(result),
        Err(err_msg) => panic!("{err_msg}"),
    }

    remove_file(OUTPUT_FILE_NAME).unwrap();
}

#[test]
#[should_panic]
fn include_6() {
    const OUTPUT_FILE_NAME:&str = "tests/include/test_structure_6/output.txt";

    let mut gravity = Gravity::new_verbose(OUTPUT_FILE_NAME.into());
    gravity.include(None, Path::new("tests/include/test_structure_6/input.txt"), "");

    match check_file_equality(
        OUTPUT_FILE_NAME,
        "tests/include/test_structure_6/correct_output.txt"
    ) {
        Ok(result) => assert!(!result),
        Err(err_msg) => panic!("{err_msg}"),
    }

    remove_file(OUTPUT_FILE_NAME).unwrap();
}

#[test]
#[should_panic]
fn include_7() {
    const OUTPUT_FILE_NAME:&str = "tests/include/test_structure_7/output.txt";

    let mut gravity = Gravity::new(OUTPUT_FILE_NAME.into());
    gravity.include(None, Path::new("tests/include/test_structure_7/input.txt"), "");

    match check_file_equality(
        OUTPUT_FILE_NAME,
        "tests/include/test_structure_7/correct_output.txt"
    ) {
        Ok(result) => assert!(result),
        Err(err_msg) => panic!("{err_msg}"),
    }

    remove_file(OUTPUT_FILE_NAME).unwrap();
}

#[test]
fn include_8() {
    const OUTPUT_FILE_NAME:&str = "tests/include/test_structure_8/output.txt";

    let mut gravity = Gravity::new_verbose(OUTPUT_FILE_NAME.into());
    gravity.include(None, Path::new("tests/include/test_structure_8/input.txt"), "");

    match check_file_equality(
        OUTPUT_FILE_NAME,
        "tests/include/test_structure_8/correct_output.txt"
    ) {
        Ok(result) => assert!(result),
        Err(err_msg) => panic!("{err_msg}"),
    }

    remove_file(OUTPUT_FILE_NAME).unwrap();
}