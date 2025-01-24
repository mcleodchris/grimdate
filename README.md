# Grimdate

A super simple command-line utility which does one thing: return a date/time formatted to use the [Imperial Dating System](https://wh40k.lexicanum.com/wiki/Imperial_Dating_System) from Warhammer 40,000.

Without any input it will use the current system date and time. All outputs are normalised to UTC. See below for sample [usage](#usage).

## Building

To build the project, you need to have Rust installed. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).

Once Rust is installed, you can build the project using the following command:

```sh
cargo build --release
```

## Installing

### From Source

To install the project from source, run the following command:

```sh
cargo install --path .
```

### From Packaged Releases

You can download the pre-built binaries from the [releases page](https://github.com/mcleodchris/grimdate/releases). Choose the appropriate binary for your operating system and architecture, and extract it to a location in your `PATH` for your terminal to find it.

## Usage

Grimdate supports several command-line options:

- `--no-spaces` / `-S`: Removes spaces from the output.
- `--is-iss` / `-i`: Changes the leading 0 to 1.
- `--date` / `-d`: Specify the input date (format: `YYYY-MM-DDTHH:MM:SSZ`).
- `--no-time` / `-T`: Returns the date without the local time.

Example usage:

```sh
grimdate # output: 0 064 025.M3//10:53 local
grimdate --no-spaces # output: 0064025.M3//10:54 local
grimdate --is-iss # output: 1 064 025.M3//10:54 local
grimdate --date "2023-10-05T15:30:00+03:00" # output: 0 759 023.M3//12:30 local
grimdate --no-time # output: 0 064 025.M3

# options can be combined, e.g.,
grimdate -iSTd "2023-10-05T15:30:00+03:00" # output: 1759023.M3
```

