<div align="center">

# schema-util

<a href="https://github.com/octocat-rs/schema-util/actions/workflows/tests.yml"><img src="https://github.com/octocat-rs/schema-util/actions/workflows/tests.yml/badge.svg?branch=main" alt="test status"></a>
<a href="https://matrix.to/#/#octocat:matrix.org" rel="noopener" target="_blank"><img src="https://img.shields.io/badge/Octocat-ffffff?style=flat&logo=Matrix&logoColor=black" alt="Chat on Matrix"></a>
<a href="https://discord.gg/Yq7aDSpfRg"> <img src="https://img.shields.io/discord/947629739219238962?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8" alt="Discord Server"> </a>

</div>

## About

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