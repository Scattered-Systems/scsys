/*
    Appellation: fs <module>
    Contrib: @FL03
*/

/// This function converts the file found at path (fp) into a [Vec<String>]
#[cfg(feature = "std")]
pub fn file_to_vec(fp: String) -> std::io::Result<Vec<String>> {
    use std::io::BufRead;
    // open the file
    let file_in = std::fs::File::open(fp)?;
    // initialize a new buffered reader
    let file_reader = std::io::BufReader::new(file_in);
    // read the file line by line and collect into a Vec<String>
    // filtering out any errors in the process
    let lines = file_reader.lines().filter_map(Result::ok);
    // collect the lines into a Vec<String>
    Ok(lines.collect::<Vec<_>>())
}
