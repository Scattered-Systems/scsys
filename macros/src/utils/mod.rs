/*
    Appellation: data <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

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

#[macro_export]
macro_rules! dframe {
    ( $( ($x:expr, $y:expr) )*, ) => {
        (
            let mut tmp = Vec::new();

            $( tmp.push(($x, $y)))*

            tmp
        )
    }
}
