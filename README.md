# jsonfmt

Format a JSON document with two-space indentation.

## Build

```sh
cargo build
```

## Usage

Pass a file path or pipe JSON into standard input.

```sh
./target/debug/jsonfmt input.json
printf '%s\n' '{"items":[1,2]}' | ./target/debug/jsonfmt
```

Formatted JSON is written to standard output. Invalid JSON produces an error on standard error and exits with a non-zero status.
