#!/usr/bin/env bash

OS="$1"
TARGET="$2"
RELEASE_VERSION="$3"

if [ "$OS" = "windows-2022" ]; then
  7z a -tzip "zman-$RELEASE_VERSION-$TARGET.zip" zman-"$RELEASE_VERSION"/
else
  tar -czvf zman-"$RELEASE_VERSION"-"$TARGET".tar.gz zman-"$RELEASE_VERSION"/
  shasum -a 512 zman-"$RELEASE_VERSION"-"$TARGET".tar.gz >zman-"$RELEASE_VERSION"-"$TARGET".tar.gz.sha512
fi
