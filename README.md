<div align="center">

# schema-util

</div>

A slightly cursed utility for converting GitHub's schema JSONs into Rust structs

## Usage Example

```bash
# Converts test.json and writes the resulting struct to out.rs
schema-util -f test.json -o out.rs 
```

## Arguments

```bash
USAGE:
    schema-util [OPTIONS] --file <FILE>

OPTIONS:
    -f, --file <FILE>        Name of the file to read
    -h, --help               Print help information
    -o, --output <OUTPUT>    Name of the file to write the struct to. If this is not set, the struct
                             will be printed to stdout
    -V, --version            Print version information
```