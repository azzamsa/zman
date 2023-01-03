#!/usr/bin/env bash

OS="$1"
TARGET="$2"
RELEASE_VERSION="$3"

TARGET_DIR=zman-"$RELEASE_VERSION"/

mkdir "$TARGET_DIR"

bin="zman"
if [ "$OS" = "windows-2022" ]; then
  bin="zman.exe"
fi
cp "target/$TARGET/release/$bin" "$TARGET_DIR"
