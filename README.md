# Lorem Ipsum Generator (Rust)

A command-line Lorem Ipsum generator that accepts the desired number of words.
It uses the much larger collection in `text.txt` as its source and repeats the
collection when the requested word count exceeds the available text.

## Usage

Pass the number of words as a command-line argument:

```bash
cargo run -- 20
```

When using the compiled binary:

```bash
./target/release/lorem-ipsum-generator 30
```

Run the application without an argument to generate the default of 20 words:

```bash
./target/release/lorem-ipsum-generator
```

The application does not request interactive input.

## Testing

```bash
cargo test
```
