_list:
    @just --list

# Lint workspace.
clippy:
    cargo clippy --workspace --all-targets -- -D warnings

# Lint workspace and watch for changes.
clippy-watch:
    cargo watch -- cargo clippy --workspace --all-features

# Apply possible linting fixes in the workspace.
clippy-fix *args:
    cargo clippy --workspace --all-features --fix {{ args }}
    cargo +nightly fmt

# Test workspace.
test:
    cargo test --workspace --all-features

# Document workspace.
doc:
    RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features

# Document workspace and watch for changes.
doc-watch:
    RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features --open
    cargo watch -- RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features

# Check project formatting and lints.
check:
    just --unstable --fmt --check
    prettier --check $(fd --hidden -e=md -e=yml)
    taplo lint
    cargo +nightly fmt -- --check
    @just clippy
    cargo msrv verify

# Format project.
fmt:
    just --unstable --fmt
    nix fmt
    prettier --write $(fd --hidden -e=md -e=yml)
    taplo format
    cargo +nightly fmt
