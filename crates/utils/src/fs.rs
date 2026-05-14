/*
    Appellation: fs <module>
    Contrib: @FL03
*/

/// This function converts the file found at path (fp) into a [Vec<String>]
pub fn file_to_vec(fp: String) -> std::io::Result<Vec<String>> {
    use std::io::BufRead;
    // open the file
    let file_in = std::fs::File::open(fp)?;
    // initialize a new buffered reader
    let file_reader = std::io::BufReader::new(file_in);
    // initialize a new buffer to hold the lines
    let mut buffer = Vec::new();
    // read the file line by line and collect into a Vec<String>
    // filtering out any errors in the process
    file_reader.lines().for_each(|line| match line {
        Ok(l) => buffer.push(l),
        Err(e) => eprintln!("Error reading line: {}", e),
    });
    // collect the lines into a Vec<String>
    Ok(buffer)
}
