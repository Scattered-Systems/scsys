/*
    Appellation: actors <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[macro_export]
macro_rules! appellation {
    ( $( $x: expr ),* ) => {
        (
            let mut tmp = Vec::new();
            $( tmp.push($x) )*

            tmp
        )
    }
}