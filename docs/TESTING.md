# Regression testing

Run the full workspace checks from the repository root:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-features -- -D warnings
cargo test --workspace --all-features
cargo check -p hmleq --no-default-features
```

The integration suite includes regression cases for:

- Indexed roots written as `sqrt <degree> of <radicand>`.
- Scripted limits and large operators in display style.
- Display-sized fractions and compact fractions inside scripts.
- Nested `LEFT` / `RIGHT` delimiters and nested matrices.
- Symbol aliases including `prime`, `cap`, `cup`, and `rarrow`.
- Full-width overlines for segment notation such as `bar AD`.

When adding a regression case, keep the input script and expected LaTeX
independent of copyrighted source documents.
