job("Login & Test (crates)") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/tags/v*.*.*"
            }
        }
        schedule { cron("0 8 * * *") }
    }
    container(displayName = "Rust", image = "rust") {
        env["CARGO_REGISTRY_TOKEN"] = Secrets("cargo_registry_token")
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
        env["TOKEN"] = Secrets("cargo_registry_token")

        shellScript {
            interpreter = "/bin/bash"
            content = """
                cargo publish --all-features --color always --jobs 1 --token ${'$'}TOKEN --verbose -p scsys-core
                cargo publish --all-features --color always --jobs 1 --token ${'$'}TOKEN --verbose -p scsys-crypto
                cargo publish --all-features --color always --jobs 1 --token ${'$'}TOKEN --verbose -p scsys-derive
                cargo publish --all-features --color always --jobs 1 --token ${'$'}TOKEN --verbose -p scsys-macros
                cargo publish --all-features --color always --jobs 1 --token ${'$'}TOKEN --verbose -p scsys
            """
        }
    }
}