#!/bin/bash
set -e

while getopts w: flag; do
  case "${flag}" in
    w) local_workspace_path=${OPTARG} ;;
    *) throw 'Unknown argument' ;;
  esac
done

echo "local_workspace_path=${local_workspace_path}"

cp -r .devcontainer/certs/ /usr/local/share/ca-certificates/
update-ca-certificates
