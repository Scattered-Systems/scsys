#[cfg(test)]
mod tests {
    #[cfg(feature = "actors")]
    use scsys::actors::agents::Agent;

    #[test]
    fn lib_compiles() {
        let a = Agent::new(String::new());
        let b = Agent::from(&a);
        assert_eq!(&a, &b)
    }
}
