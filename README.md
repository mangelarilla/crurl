[![crates.io](https://img.shields.io/crates/v/crurl)](https://crates.io/crates/crurl)

# crurl 

Simplified http client, made with Rust!

```
crurl 0.2.0
Simplified http client

USAGE:
    crurl [FLAGS] [OPTIONS] <url>

FLAGS:
        --help            Prints help information
    -h, --headers-only    Response headers only
        --post            Request with POST verb
        --put             Request with PUT verb
    -s, --show-request    Display the request
    -V, --version         Prints version information

OPTIONS:
    -B, --body <body>             Set request body
    -H, --headers <headers>...    Set request headers. Ex: "Accept=*/*"

ARGS:
    <url>    The target url to request
```

## Installation

Just via cargo for now...

### Cargo

Get [rustup](https://rustup.rs) installed locally and then run `cargo install crurl`, which will download the binary package from [crates.io](https://crates.io) ready to use
