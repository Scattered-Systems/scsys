/*
    Appellation: casing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use scsys_utils::casing::to_snakecase;

#[test]
fn test_snakecase() {
    let s = "HelloWorld";

    assert_eq!(to_snakecase(s), "hello_world");
}