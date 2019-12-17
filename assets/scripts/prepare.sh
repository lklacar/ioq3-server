#!/usr/bin/env bash

rm -rf baseq3
mkdir -p repacked
node repak/repak.js --src assets --dest repacked
mv repacked/* .

mkdir -p baseq3
cd baseq3 || exit
cp -a ../paks/baseq3/. .