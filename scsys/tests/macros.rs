/*
    Appellation: macros <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[cfg(test)]
#[cfg(feature = "macros")]
mod tests {
    use scsys::{extend_path, shared};

    #[test]
    fn test_extend_path() {
        let a = extend_path!["/tmp"; ["daemon.out", "daemon.err", "pid.test"]];
        let b = vec!["/tmp/daemon.out", "/tmp/daemon.err", "/tmp/pid.test"];

        assert_eq!(a, b)
    }

    #[test]
    fn test_shared() {
        let a = shared!(String::from("Hello, World!"));
        assert!(a.lock().is_ok());
    }
}
