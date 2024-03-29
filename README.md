# rrr (Really Rapid Requestor) 🚀

rrr is a CLI tool designed to fetch a bunch of URLs rapidly. That's pretty much it. It can save responses to files in a folder, or output them to STDOUT for piping.

## Features 🌟
- **Custom HTTP Methods**: Choose the HTTP method for your requests.
- **Save Responses**: Automatically save response bodies to a specified directory.
- **Ignore Specific Status Codes**: Skip processing responses with certain HTTP status codes.
- **STDOUT Output**: Directly print response bodies to STDOUT for further processing.

## Usage 🛠

```
rrr [OPTIONS]
```

### Options:

- `-m, --method <METHOD>`: Optional HTTP method to use for requests. Default is `GET`.
- `-d, --directory <DIRECTORY>`: Optional directory to save response bodies. Default is `responses`.
- `-i, --ignore <IGNORE>`: Optional comma-separated list of HTTP response status codes to ignore (e.g., `404,403,500`).
- `-o, --stdout`: Print response bodies to STDOUT instead of saving them.
- `-h, --help`: Display help information.
- `-V, --version`: Display the version number.

### Examples:

- Chain with other tools and save responses:
    ```
    cat ranges.txt | httpx | rrr -d responses
    ```
- Filter out specific status codes and print responses:
    ```
    cat urls.txt | rrr -i 404,403,500 -o > responses.txt
    ```
- Find interesting responses containing by piping to tools like Ripgrep:
    ```
    cat ranges.txt | daship | httpx | rrr -o | rg "hackme" > interesting.txt
    ```

## Get Started 🔥

### Install Fro Prebuilt Binaries (recommended)
TODO: setup build and release github action.
Grab the prebuilt binary for your OS from the [releases]().

### Install From Source
Ensure you have Rust installed, then clone the repo, install with `cargo install --path .`.


Happy requesting! 🌐✨
