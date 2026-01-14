#!/bin/bash

set -ex

LIBPOD_SWAGGER_URL="https://storage.googleapis.com/libpod-master-releases"
LIBPOD_API_VERSION="v5.4.2"
LIBPOD_SPEC_FILE="swagger-${LIBPOD_API_VERSION}.yaml"
LIBPOD_FULL_URL="${LIBPOD_SWAGGER_URL}/${LIBPOD_SPEC_FILE}"
RUSTGEN="https://github.com/vv9k/swagger-rustgen.git"
BUILD_DIR=build
BASE_DIR=$PWD

mkdir $BUILD_DIR || true

cd $BUILD_DIR
echo $PWD

curl -LO $LIBPOD_FULL_URL

git clone $RUSTGEN || true
cd swagger-rustgen
cargo build --release
cd $BASE_DIR

cat base/models.rs > lib/src/models.rs

$BUILD_DIR/swagger-rustgen/target/release/swagger-gen generate models $BUILD_DIR/$LIBPOD_SPEC_FILE >> lib/src/models.rs

cd lib

cargo fmt

sed -r -i 's/(PortMap = HashMap<String, )(Vec<PortBinding>)/\1Option<\2>/g' src/models.rs

# issue 148
sed -r -i 's/((ports|port_bindings): Option<HashMap<String, )(Vec<InspectHostPort>)/\1Option<\3>/g' src/models.rs

cargo fmt
