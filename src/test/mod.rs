use std::fs;

use crate::{addresses_to_result_with_csv_crate, addresses_to_result_with_own_csv_parser};

#[test]
fn test_addresses_to_result_with_own_csv_parser() {
    let file_content = fs::read_to_string("tests/assets/addresses.csv");
    assert!(file_content.is_ok());

    let file_content = file_content.unwrap();

    let result = addresses_to_result_with_own_csv_parser(file_content);

    assert!(result.is_ok());
    //let result = result.unwrap();
}

// CSV crate seems to be returning only prefixes of some colum names, and even worse: it's inconsistent about it!
#[test]
fn test_addresses_to_result_with_csv_crate() {
    let file_content = fs::read_to_string("tests/assets/addresses.csv");
    assert!(file_content.is_ok());

    let file_content = file_content.unwrap();
    let bytes = file_content.as_bytes();

    let result = addresses_to_result_with_csv_crate(bytes);

    assert!(result.is_ok());
    //let result = result.unwrap();
}
