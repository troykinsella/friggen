#!/usr/bin/env bash

set -euo pipefail

owner=troykinsella
repo=friggen

# Functions

usage() {
  {
    echo "Usage: $0 [options]"
    echo "Install the latest version of friggen!"
    echo
    echo "Options:"
    echo -e "-h|--help\tThis business"
    echo -e "-t|--target\tThe directory path in which friggen will be installed [Default: /usr/local/bin]"
  } >&2
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
      echo "Friggen doesn't support your silly system. Not even sorry." >&2
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

main() {
  while test $# -gt 0; do
    case "$1" in
      -h|--help)
        usage
        exit 0
        ;;
      -t|--target)
        target="$2"
        shift
        ;;
      *)
        echo "What's this you're passing me? pff... '$1'" >&2
        usage
        exit 1
        ;;
    esac
    shift
  done

  target="${target:-/usr/local/bin}"
  dest="$target/friggen"
  version=$(latest_version)
  archive_url="https://github.com/${owner}/${repo}/releases/download/${version}/$(archive_name $version)"
  temp_dir=$(mktemp -d || mktemp -d -t tmp)

  echo "
  ┏  •
  ╋┏┓┓┏┓┏┓┏┓┏┓
  ┛┛ ┗┗┫┗┫┗ ┛┗
       ┛ ┛
  "

  if ! [[ -d $target ]]; then
    echo "Yeah $target doesn't exist." >&2
    echo "I'm not going to do everything for you." >&2
    exit 1
  fi

  if [[ -f $dest ]]; then
    echo "Uh... something already exists at $dest." >&2
    echo "The heck are ya doing?" >&2
    exit 1
  fi

  echo "Fetching $archive_url"
  curl -fSsL "$archive_url" | tar -zxf - -C "$temp_dir"

  install -m 755 "$temp_dir/friggen" "$dest"
  rm -rf "$temp_dir"

  echo "Friggen installed! » $dest"
}

# Main

main "$@"
