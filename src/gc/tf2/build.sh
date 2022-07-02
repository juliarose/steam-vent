#!/usr/bin/env bash

rsprotoc() { protoc "$@" --rust_out ../ --proto_path=./ --proto_path=/usr/include --proto_path=/usr/local/include ;}

cd protos

echo -n "" > ../mod.rs

for filename in *.proto; do
    rsprotoc $filename

    mod="${filename%.*}";
    mod="${mod//\./_}";
    echo "pub mod ${mod};" >> ../mod.rs
done