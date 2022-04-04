#!/bin/sh

# I happen to be running Linux, so I use this script to easily compile for the
# apple darwin target

# if the osxcross tools have already been installed, skip
if [ ! -d "osxcross/" ];then
    git clone https://github.com/tpoechtrager/osxcross
    cd osxcross
    wget -nc https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz
    mv MacOSX10.10.sdk.tar.xz tarballs/
    UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh
fi

# add osxcross tools to path and cargo build
# first cli argument can be --release
PATH="$(pwd)/osxcross/target/bin:$PATH" \
CC=o64-clang \
CXX=o64-clang++ \
cargo build $1 --target x86_64-apple-darwin
