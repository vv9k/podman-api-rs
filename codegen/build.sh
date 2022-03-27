#!/bin/bash

set -ex

mvn clean compiler:compile generate-resources

cd ./target/gen

cargo fmt

# https://github.com/vv9k/podman-api-rs/issues/111
sed -i -r "s/(port_bindings: Option<HashMap<String, )(Vec<InspectHostPort>>>)/\1Option<\2>/g" src/models.rs

# https://github.com/vv9k/podman-api-rs/issues/113
sed -i -r "s/(type PortMap = HashMap<String, )String>/\1Option<Vec<PortBinding>>>/g" src/models.rs

cargo fmt

