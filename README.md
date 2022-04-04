# pcommit
[![build](https://github.com/sudosmile/pcommit/actions/workflows/build.yml/badge.svg)](https://github.com/sudosmile/pcommit/actions/workflows/build.yml)
[![release](https://github.com/sudosmile/pcommit/actions/workflows/release.yml/badge.svg)](https://github.com/sudosmile/pcommit/actions/workflows/release.yml)

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
