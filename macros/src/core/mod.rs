/*
    Appellation: core <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::utils::*;

mod utils {
    #[macro_export]
    macro_rules! printer {
        ( $( $x: expr ),* ) => {
            {
                $(
                    println!("{:#?}", $x);
                )*
            }
        };
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn test_printer_macro() {
            let res = Some(printer![("hello", vec!["world"])]);
            assert_eq!(res, Some(()))
        }
    }
}


