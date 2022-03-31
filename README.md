# pcommit
[![Rust](https://github.com/sudosmile/pcommit/actions/workflows/rust.yml/badge.svg?branch=main&event=push)](https://github.com/sudosmile/pcommit/actions/workflows/rust.yml)

A commit message formatter written in rust,
adhering to this [commit message convention](https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13)


## installation:

if the rust toolchain isn't installed:
```sh
$ curl https://sh.rustup.rs -sSf | sh
```

then, to install pcommit:
```sh
$ git clone "https://github.com/sudosmile/pcommit.git"
$ cd smilefetch
$ cargo install --path .
```


## usage:

run pcommit and follow the prompts, or :
```sh
$ pcommit --help
```
