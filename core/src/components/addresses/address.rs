/*
    Appellation: address <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use std::net;

pub enum Address<T: ToString> {
    Tcp(T),
    Ip(T),
}
