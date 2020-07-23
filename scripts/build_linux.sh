#!/bin/bash

# Build rust executable
cargo build --release

# Create clean build directory & AppImage's "AppDir" inside it
rm -rf build/linux
mkdir -p build/linux
mkdir -p build/linux/hello.AppDir

# Copy executable and resources to "AppDir"
cp target/release/hello build/linux/hello.AppDir/AppRun
cp -r resources build/linux/hello.AppDir

# Rearrange some files
cd build/linux/hello.AppDir
mv resources/logo.png hello.png
mv resources/linux/hello.desktop .

# Create the final .AppImage
cd ..
appimagetool-x86_64.AppImage hello.AppDir

# Delete build artifacts
rm -rf hello.AppDir
