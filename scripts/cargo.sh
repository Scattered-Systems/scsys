#!/usr/bin/env bash
# Sprint cleanup utility — run after clearing context between sprints.
# For builds, use: cargo xtask
#
# Defaults: fmt + lint. Pass flags to add/remove steps.

set -euo pipefail
cd "$(dirname "$0")/.."

usage() {
    cat <<EOF
Usage: $(basename "$0") [flags]

Sprint-context reset utility. Formats, lints, optionally cleans and docs.
For builds use: cargo xtask

  -b, --build      Run cargo build
  -c, --clean      Run cargo clean (expensive; skipped by default)
  -d, --doc        Generate docs  (target/doc/axiom/all.html)
  -u, --update     Run cargo update (default: on; disable with --no-update)
      --no-update  Skip cargo update
  -f, --fmt        Run cargo fmt --all (default: on; disable with --no-fmt)
      --no-fmt     Skip cargo fmt
  -l, --lint       Run cargo clippy (default: on; disable with --no-lint)
      --no-lint    Skip cargo clippy
  -t, --test       Run cargo test --workspace --features full (default: off)
  -a, --all        Enable everything: --clean --doc --update --fmt --lint --test
  -h, --help       Show this message
EOF
}

DO_BUILD=true
DO_CHECK=true
DO_CLEAN=false
DO_CLIPPY=true
DO_DOC=true
DO_FMT=true
DO_FIX=true
DO_LINT=true
DO_TEST=false
DO_UPDATE=true

while [[ $# -gt 0 ]]; do
    case "$1" in
        -b|--build)    DO_BUILD=true   ;;
        --no-build)    DO_BUILD=false  ;;
        -c|--clean)    DO_CLEAN=true   ;;
        -d|--doc)      DO_DOC=true     ;;
        -u|--update)   DO_UPDATE=true  ;;
        --no-update)   DO_UPDATE=false ;;
        -f|--fmt)      DO_FMT=true     ;;
        --no-fmt)      DO_FMT=false    ;;
        -l|--lint)     DO_CLIPPY=true  ;;
        --no-lint)     DO_CLIPPY=false ;;
        -t|--test)     DO_TEST=true    ;;
        --check)       DO_CHECK=true   ;;
        --no-check)    DO_CHECK=false  ;;
        --clippy)      DO_CLIPPY=true  ;;
        --no-clippy)   DO_CLIPPY=false ;;
        --fix)         DO_FIX=true ;;
        --no-fix)      DO_FIX=false ;;
        -a|--all)
            DO_CHECK=true
            DO_CLIPPY=true
            DO_FIX=true
            DO_CLEAN=true
            DO_BUILD=true
            DO_DOC=true
            DO_UPDATE=true
            DO_FMT=true
            DO_TEST=true
            ;;
        -h|--help)     usage; exit 0 ;;
        *) echo "Unknown flag: $1" >&2; usage >&2; exit 1 ;;
    esac
    shift
done

step() { echo "→ $*"; }

if $DO_CLEAN; then
    step "clean"
    cargo clean
fi

if $DO_UPDATE; then
    step "update"
    cargo update
fi

if $DO_CHECK; then
    step "check"
    cargo check --lib --locked --workspace --features full
fi

if $DO_CLIPPY; then
    step "lint"
    cargo clippy --workspace --features full -- -D warnings
fi

if $DO_FIX; then
    step "fix"
    cargo fix --allow-dirty --lib --workspace --features full
fi

if $DO_FMT; then
    step "fmt"
    cargo fmt --all
fi


# ------ doc/build/test/bench

if $DO_DOC; then
    step "cargo doc --locked --workspace --no-deps --document-private-items"
    cargo doc --locked --workspace --no-deps --document-private-items
    echo "   open target/doc/axiom/all.html"
fi

if $DO_BUILD; then
    step "cargo build"
    cargo build -r --workspace --locked --bins
fi

if $DO_TEST; then
    step "cargo test --workspace --features full"
    cargo test --lib --workspace --features full
fi

echo "✓ done"
