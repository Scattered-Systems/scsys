/*
    Appellation: scsys-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[macro_export]
macro_rules! join {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp = String::new();
            $(
                tmp.push_str($x);
            )*
            tmp
        }
    };
}

#[macro_export]
macro_rules! extend_path {
    ($(
        $x:expr;
        [ $( $y:expr ),* ]
    );*) => {
        vec![
            $($(
            format!("{}/{}", $x, $y)
            ),*),*
        ]
    }
}
