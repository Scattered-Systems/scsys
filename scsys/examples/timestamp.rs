use scsys::{Result, Timestamp};

fn main() -> Result {
    let ts = Timestamp::now();
    let str_ts = ts.to_rfc3339();
    assert_eq!(Timestamp::from(&ts), Timestamp::try_from(str_ts.clone()).unwrap());

    println!("{}", str_ts);

    Ok(())
}