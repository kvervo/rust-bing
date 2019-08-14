#!/bin/bash

set -e

TARGET=target/release
RESOURCES=resources
APP_NAME=rust-bing
MACOS_APP_DIR=$TARGET/$APP_NAME.app

MACOS_APP_NAME=rust-bing

echo "Creating app directory structure"
rm -rf $MACOS_APP_DIR
mkdir -p $MACOS_APP_DIR/Contents/MacOS

cargo rustc \
    --verbose \
    --release

#By default on Linux and macOS, symbol information is included in the compiled .elf file.
# This information is not needed to properly execute the binary.
# To remove this, run strip on the .elf file:
echo "Stripping symbol information from binary"
strip $TARGET/$APP_NAME

echo "Copying binary"
cp $TARGET/$APP_NAME $MACOS_APP_DIR/Contents/MacOS/$APP_NAME

echo "Copying resources directory"
mkdir $MACOS_APP_DIR/Contents/Resources/
cp $RESOURCES/icon.icns $MACOS_APP_DIR/Contents/Resources/icon.icns

echo "Copying plist directory"
cp $RESOURCES/Info.plist $MACOS_APP_DIR/Contents/Info.plist

# echo "Creating dmg"
# mkdir $MACOS_APP_NAME
# mv $MACOS_APP_DIR $MACOS_APP_NAME
# ln -s /Applications $MACOS_APP_NAME/Applications
# rm -rf $MACOS_APP_NAME/.Trashes

# FULL_NAME=$APP_NAME-$OS-$MACHINE-$SUFFIX

# hdiutil create uploads/$FULL_NAME.dmg -srcfolder $MACOS_APP_NAME -ov
# rm -rf $MACOS_APP_NAME