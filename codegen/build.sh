#!/bin/bash

set -ex

mvn clean compiler:compile generate-resources

cd ./target/gen
cargo fmt
