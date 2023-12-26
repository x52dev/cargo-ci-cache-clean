# `cargo-ci-cache-clean`

> Clean up unnecessary Cargo artifacts to improve CI caching performance

# Install

In GitHub Actions:

```yaml
# jobs.<job_name>.steps:

- name: Install cargo-ci-cache-clean
  uses: taiki-e/install-action@v2
  with:
    tool: cargo-ci-cache-clean
```

With [`cargo-binstall`]:

```console
$ cargo binstall cargo-ci-cache-clean
```

From source:

```console
$ cargo install cargo-ci-cache-clean
```

# Usage

```console
cargo-ci-cache-clean
```

[`cargo-binstall`]: https://github.com/cargo-bins/cargo-binstall
