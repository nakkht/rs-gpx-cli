# rs-gpx-cli
[![Build Status](https://travis-ci.com/nakkht/rs-gpx-cli.svg?branch=develop)](https://travis-ci.com/nakkht/rs-gpx-cli)

CLI tool for generating and manipulating gpx files:

Features:
- [x] Generating gpx file with timestamps based on speed

## Environment 

To be able to build sources locally, you will have to setup [Rust development environment](https://www.rust-lang.org/learn/get-started)

## Build 

- Clone repository
- Run `cargo build` for debug build and `cargo build --release` for release build

## Install

### Source

Once repository cloned, run the following in the repository root:

```
cargo install --path ./
```

## Usage

* `rs-gpx -i absolute_path_to_source -o absolute_path_to_output -s 50`
* `rs-gpx -h, --help`
* `rs-gpx -V, --version`

## Author
* [Paulius Gudonis](https://pgu.dev)

## License
This repository is under the [MIT license](https://github.com/nakkht/rs-gpx-cli/blob/master/LICENSE).
