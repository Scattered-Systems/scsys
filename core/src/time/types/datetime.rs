/*
    Appellation: datetime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u16,
    pub microsecond: u16,
    pub nanosecond: u16,
}

pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

#[cfg(test)]
mod tests {}
