# Frisk

Frisk is an efficient command-line tool to search for file(s) / folder(s) in your file system

## Installation

Frisk is published on [crates.io](https://crates.io/). First, install the package manager [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and then install frisk as a binary crate

```bash
cargo install frisk
```

## Usage

```bash
$ frisk abc.txt
No path specified! So, starting search from  /
/tmp/abc.txt

1731660 items have been searched in 0 seconds to spot 1 item(s)

$ touch /tmp/def.txt

$ frisk def.txt --dir_path /tmp/
/tmp/def.txt

26 items have been searched in 0 seconds to spot 1 item(s)

$ frisk --help
Spot file(s) or folder(s) in your filesystem by its name

Usage: frisk [OPTIONS] <file_name>

Arguments:
  <file_name>  Name or regex pattern of file(s) or folder(s)

Options:
      --dir_path <dir_path>  Path to dir from where search should begin
      --follow_links         Whether to follow symbolic links or NOT in the search
      --skip_hidden          Whether to include hidden files or NOT in the search
  -h, --help                 Print help information
  -V, --version              Print version information
```

## License

[MIT](https://choosealicense.com/licenses/mit/)