/*
    Appellation: context <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

pub trait Context<Cnf> {
    fn context(&self) -> Self;
    fn settings(&self) -> Cnf;
}
