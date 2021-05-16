# A Color API written in rust

This is a sample project that implement the Color API with rust/cargo. It
is useful to demonstrate [go-crzy/crzy](https://github.com/go-crzy/crzy). To
test the project, we assume you have rust installed. 

To test the project, run:

```bash
cargo test
```

To build the project, run 

```bash
cargo build
```

the artifact is named `color` and located in `target/release` or `target/debug`
depending on the profile you'll use.

To use it, simply run:

```
export PORT=8080
./color
```
