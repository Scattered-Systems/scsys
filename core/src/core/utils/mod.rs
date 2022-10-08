/*
    Appellation: utils <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{
    cnf::collect_config_files,
    fs::file_to_vec,
    time::{chrono_datetime_now, chrono_into_bson, Timestamp},
};

mod cnf;
mod fs;
mod time;
