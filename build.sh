#!/usr/bin/env bash

echo "Cleaning build folder"
rm -rf build
mkdir build

echo "Building server executable"
cd build || exit
cmake -DCMAKE_BUILD_TYPE=Release ..
make

echo "Copying proxy"
cp ../quakejs/proxy . -r
cd proxy || exit
yarn install
cd ../

echo "Copying repack"
cp ../quakejs/repak . -r
cd repak || exit
yarn install
cd ../

echo "Copying content server"
cp ../quakejs/content . -r
cd content || exit
yarn install
cd ../

mkdir assets/baseq3 -p
echo "PLACE_NEW_CONTENT_HERE" >> assets/baseq3/PLACE_NEW_CONTENT_HERE

echo "Copying original paks"
cp ../assets/paks . -r

echo "Copying bin"
cp -a ../assets/bin/. .

echo "Copying scripts"
cp -a ../assets/scripts/. .


echo "Cleaning up"
rm -rf CMakeFiles
rm cmake_install.cmake
rm CMakeCache.txt
rm Makefile