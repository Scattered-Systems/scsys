/*
    Appellation: scsys-macros <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

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


#[macro_export]
macro_rules! dict {
    ( $( ($x:expr, $y:expr) ),* ) => {
        {
            let mut tmp = std::collections::BTreeMap::new();
            $(
                tmp.insert($x, $y);
            )*
            tmp
        }
    };
}


#[cfg(test)]
mod tests {
    use super::{dict, printer};

    #[test]
    fn test_dict_macro() {
        let a = dict![("hello", vec!["world"])];
        let b = a.clone();
        assert_eq!(a, b)
    }

    #[test]
    fn test_printer_macro() {
        let res = Some(printer![("hello", vec!["world"])]);
        assert_eq!(res, Some(()))
    }
}