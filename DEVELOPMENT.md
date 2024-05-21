# Development Setup

This is a standard Rust library so the setup is also pretty standard. Just clone the repo & work on it.

## Adding snapshot test

vi-rs uses cargo-insta for snapshot testing. To add a snapshot, open `testdata/input/<snapshot_file>.txt` and add your test words/sentences there. After that, run `cargo insta test` to get an output for your newly added test words/sentences. That command will also be used to verify if the current test inputs all produce the same output after code changes.

If the output of the snapshot test is different, you can review those differences by running `cargo insta review`. This is a pretty standard `cargo-insta` workflow.

## Run benchmark

Just run `cargo bench` and watch the magic. The code for the benchmark is located in `benches` folder.

## Publish

To publish a new version, follow these steps:

1. Work out what version you want to publish using semver.
2. Update `Cargo.toml` with the new version.
3. Run `git stage Cargo.toml`
4. Run `git tag <version>`
5. Run `cargo build --release`
6. Run `cargo publish`
7. Run `git push --follow-tags`
