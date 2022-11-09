job("Login & Test (crates)") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/heads/main"
                +"refs/tags/v*.*.*"
            }
        }
        schedule { cron("0 8 * * *") }
    }
    container(displayName = "Rust", image = "rust") {
        shellScript {
            interpreter = "/bin/bash"
            content = """
                cargo login ${'$'}CARGO_REGISTRY_TOKEN
                cargo test --all-features
            """
        }
    }
}

job("Publish (crates)") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/tags/v*.*.*"
            }
        }
    }
    container(displayName = "Rust", image = "rust") {
        shellScript {
            interpreter = "/bin/bash"
            content = """
                cargo login ${'$'}CARGO_REGISTRY_TOKEN
                cargo publish --all-features --color always --jobs 1 --verbose -p scsys-core
                cargo publish --all-features --color always --jobs 1 --verbose -p scsys-crypto
                cargo publish --all-features --color always --jobs 1 --verbose -p scsys-derive
                cargo publish --all-features --color always --jobs 1 --verbose -p scsys-macros
                cargo publish --all-features --color always --jobs 1 --verbose -p scsys
            """
        }
    }
}