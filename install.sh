#!/usr/bin/env bash

set -euo pipefail

owner=troykinsella
repo=friggen

usage() {
  {
    echo "usage: $0 [options]"
    echo "Install the latest version of friggen!"
    echo
    echo "options:"
    echo -e "-h|--help\tThis business"
    echo -e "-t|--target\tThe directory path in which friggen will be installed [Default: /usr/local/bin]"
  } >&2
  exit 0
}

archive_name() {
  local version="$1"

  system="$(uname -s)-$(uname -m)"
  case $system in
    "Darwin-x86_64")
      artifact_suffix="darwin-x86_64"
      ;;
    "Darwin-arm64")
      artifact_suffix="darwin-aarch64"
      ;;
    "Linux-x86_64")
      artifact_suffix="linux-x86_64"
      ;;
    "Linux-aarch64" | "Linux-arm64")
      artifact_suffix="linux-aarch64"
      ;;
    *)
      echo "friggen doesn't support your silly system. Not even sorry." >&2
      exit 1
  esac

  echo "friggen-${version#v}-${artifact_suffix}.tar.gz"
}

latest_version() {
  curl -fSsL "https://api.github.com/repos/${owner}/${repo}/releases" | \
    grep tag_name | \
    head | \
    cut -d'"' -f4
}


while test $# -gt 0; do
  case "$1" in
    -h|--help)
      usage
      ;;
    -t|--target)
      target="$2"
      shift
      ;;
    *)
      ;;
  esac
  shift
done

target="${target:-/usr/local/bin}"
version=$(latest_version)
archive_url="https://github.com/${owner}/${repo}/releases/download/${version}/$(archive_name $version)"
temp_dir=$(mktemp -d || mktemp -d -t tmp)

echo "
┏  •
╋┏┓┓┏┓┏┓┏┓┏┓
┛┛ ┗┗┫┗┫┗ ┛┗
     ┛ ┛
"

echo "fetching $archive_url"
curl -fSsL "$archive_url" | tar -zxf - -C "$temp_dir"

mv "$temp_dir/friggen" "$target"
chmod 755 "$target"

rm -rf "$temp_dir"

echo "friggen installed! » $target/friggen"
