# CIDR to IP List Converter

This tool converts a CIDR notation to a list of IP addresses. It supports only IPv4 addresses.

## Features

- Converts CIDR notation to a list of IPv4 addresses.
- Optionally outputs the list in a format suitable for BigQuery queries.

## Usage

### Command Line Arguments

- `--cidr`: Specify the CIDR notation you want to convert. (e.g., `--cidr 127.0.0.1/30`)
- `--query` (`-q`): Outputs the list in a format suitable for BigQuery queries. (default: `false`)

### Examples

Display IP addresses for a given CIDR:

```sh
cargo run -- --cidr 127.0.0.1/30
```

Output IP addresses in BigQuery query format:

```sh
cargo run -- --cidr 127.0.0.1/30 --query
```

## Installation

1. Ensure you have Rust installed on your machine. You can download Rust [here](https://www.rust-lang.org/tools/install).
2. Clone the repository:

    ```sh
    git clone https://github.com/raiga0310/bq-cidr.git
    cd bq-cidr
    ```

3. Run the project:

    ```sh
    cargo run -- --cidr 127.0.0.1/30
    ```

## Example Output

### Default Output

```sh
cargo run -- --cidr 127.0.0.1/30
```

```
127.0.0.0
127.0.0.1
127.0.0.2
127.0.0.3
```

### BigQuery Query Format

```sh
cargo run -- --cidr 127.0.0.1/30 --query
```

```
(
  '127.0.0.0',
  '127.0.0.1',
  '127.0.0.2',
  '127.0.0.3'
)
```
