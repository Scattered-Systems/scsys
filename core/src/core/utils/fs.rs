/*
    Appellation: fs <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use std::{fs::File, io::{self, BufRead, BufReader}};

/// This function converts the file found at path (fp) into a Vec<String>
pub fn file_to_vec(fp: String) -> io::Result<Vec<String>> {
    let file_in = File::open(fp)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
