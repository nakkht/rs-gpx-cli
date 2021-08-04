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

## Licence
This repository is under the [MIT license](https://github.com/nakkht/rs-gpx-cli/blob/main/LICENSE).

    Copyright 2021 Paulius Gudonis

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
