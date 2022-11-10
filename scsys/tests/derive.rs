/*
    Appellation: derive <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[cfg(feature = "derive")]
#[cfg(test)]
mod tests {
    use scsys::HelloWorld;

    #[derive(HelloWorld)]
    struct Pancakes;

    #[test]
    fn test_simple_derive() {

        let a = Pancakes::hello_world();
        let b = "Hello, World! My name is Pancakes".to_string();

        assert_eq!(a, b)
    }
}