/*
    Appellation: datetime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub struct Date;

pub struct Time {}

pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

#[cfg(test)]
mod tests {}
