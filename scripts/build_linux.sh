#!/bin/bash

# create build/linux
# cd
# create appdir
# copy the stuff (maybe make some path variables to make easier)

# delete appdir

rm *.AppImage
rm -rf hello.AppDir
cargo build --release
mkdir -p hello.AppDir

cp -r resources hello.AppDir

cp target/release/hello hello.AppDir/AppRun

cd hello.AppDir
mv resources/logo.png hello.png

echo '[Desktop Entry]' > hello.desktop
echo 'Name=hello' >> hello.desktop
echo 'Exec=hello' >> hello.desktop
echo 'Icon=hello' >> hello.desktop
echo 'Type=Application' >> hello.desktop
echo 'Categories=Finance;' >> hello.desktop

cd ..
appimagetool-x86_64.AppImage hello.AppDir

