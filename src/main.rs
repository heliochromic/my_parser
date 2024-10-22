use my_parser_prokhorov_bohdan::*;

pub fn main() {
    let parsed_data = list_parser::list("[1,1,2,3,5,8]");

    println!("{:?}", parsed_data);

    assert_eq!(parsed_data, Ok(vec![1,1,2,3,5,8]));
}