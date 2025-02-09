#!/bin/bash
# This script is used to publish the crate to crates.io
# It is a workaround for a difficulty with WSL on Windows.
# Basically, it copies the necessary files to a temporary directory
# and then publishes the crate from there.

# Define the temporary directory
TMP_DIR="/tmp/crate_publish"

# Create the temporary directory
mkdir -p $TMP_DIR

# Copy the necessary files to the temporary directory
cp -r src $TMP_DIR/
cp Cargo.toml $TMP_DIR/
cp Cargo.lock $TMP_DIR/
cp README.md $TMP_DIR/
cp LICENSE $TMP_DIR/

# Change to the temporary directory
cd $TMP_DIR

# Publish the crate to crates.io
cargo publish

# Clean up the temporary directory
cd -
rm -rf $TMP_DIR

echo "Crate published successfully from temporary directory."