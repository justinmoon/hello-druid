#!/bin/bash

set -e

APP_NAME=hello
MACOS_BIN_NAME=hello-bin
MACOS_APP_NAME=Hello
MACOS_APP_DIR=$MACOS_APP_NAME.app

echo "Creating build directory"
rm -rf build/macos
mkdir -p build/macos
cd build/macos
mkdir -p $MACOS_APP_DIR/Contents/MacOS

echo "Building Rust executable"
cargo build --release

echo "Copying binary"
MACOS_APP_BIN=$MACOS_APP_DIR/Contents/MacOS/$MACOS_BIN_NAME
cp ../../target/release/$APP_NAME $MACOS_APP_BIN

echo "Copying resources directory"
cp -r ../../resources $MACOS_APP_DIR/Contents/MacOS

echo "Copying launcher"
cp ../../scripts/macos_launch.sh $MACOS_APP_DIR/Contents/MacOS/$MACOS_APP_NAME

echo "Copying Icon"
mkdir -p $MACOS_APP_DIR/Contents/Resources
mv $MACOS_APP_DIR/Contents/MacOS/resources/macos/Info.plist $MACOS_APP_DIR/Contents/
# mv $MACOS_APP_DIR/Contents/MacOS/resources/macos/logo.icns $MACOS_APP_DIR/Contents/Resources/

echo "Creating dmg"
mkdir -p $MACOS_APP_NAME
cp -r $MACOS_APP_DIR $MACOS_APP_NAME/
rm -rf $MACOS_APP_NAME/.Trashes

FULL_NAME=$MACOS_APP_NAME

hdiutil create $FULL_NAME.dmg -srcfolder $MACOS_APP_NAME -ov

# Remove build artifacts
rm -rf $MACOS_APP_NAME
# rm -rf $MACOS_APP_DIR

echo "Done"
