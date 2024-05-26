#!/bin/bash
set -ex

while getopts w: flag; do
  case "${flag}" in
    w) local_workspace_path=${OPTARG} ;;
    *) throw 'Unknown argument' ;;
  esac
done

echo "local_workspace_path=${local_workspace_path}"

user_profile_path=""

full_cygpath=$(command -v cygpath) || :
if [ -n "$full_cygpath" ]; then
  if [[ "$local_workspace_path" != /* ]]; then
    local_workspace_path=$(cygpath -u "${local_workspace_path}")
  fi
fi

full_wslpath=$(command -v wslpath) || :
if [ -n "$full_wslpath" ]; then
  if [[ "$local_workspace_path" != /* ]]; then
    wsl_local_workspace_path=$(wslpath -u "${local_workspace_path}") || :
    if [ -n "$wsl_local_workspace_path" ]; then
      local_workspace_path=$wsl_local_workspace_path
    fi
  fi

  user_profile_path=$(cmd.exe /c "<nul set /p=%UserProfile%" 2> /dev/null) || :
  user_profile_path=$(wslpath -u "${user_profile_path}") || :
fi

mkdir -p "${local_workspace_path}/.profile/.vscode-server/extensions"
mkdir -p "${local_workspace_path}/.profile/.vscode-server-insiders/extensions"
mkdir -p "${local_workspace_path}/.profile/vscode/history"
mkdir -p "${local_workspace_path}/.profile/root/history"

if [ ! -f ~/.ssh/id_rsa ]; then
  if [ -z "$user_profile_path" ]; then
    touch ~/.ssh/id_rsa
  else
    if [ ! -f "${user_profile_path}/.ssh/id_rsa" ]; then
      touch "${user_profile_path}/.ssh/id_rsa"
    fi
    ln -s "${user_profile_path}/.ssh/id_rsa" ~/.ssh/id_rsa
  fi
fi

if [ ! -f ~/.ssh/config ]; then
  if [ -z "$user_profile_path" ]; then
    touch ~/.ssh/config
  else
    if [ ! -f "${user_profile_path}/.ssh/config" ]; then
      touch "${user_profile_path}/.ssh/config"
    fi
    ln -s "${user_profile_path}/.ssh/config" ~/.ssh/config
  fi
fi

if [ ! -f ~/.ssh/known_hosts ]; then
  if [ -z "$user_profile_path" ]; then
    touch ~/.ssh/known_hosts
  else
    if [ ! -f "${user_profile_path}/.ssh/known_hosts" ]; then
      touch "${user_profile_path}/.ssh/known_hosts"
    fi
    ln -s "${user_profile_path}/.ssh/known_hosts" ~/.ssh/known_hosts
  fi
fi

if [ ! -f ~/.gitconfig ]; then
  if [ -z "$user_profile_path" ]; then
    touch ~/.gitconfig
  else
    if [ ! -f "${user_profile_path}/.gitconfig" ]; then
      touch "${user_profile_path}/.gitconfig"
    fi
    ln -s "${user_profile_path}/.gitconfig" ~/.gitconfig
  fi
fi
