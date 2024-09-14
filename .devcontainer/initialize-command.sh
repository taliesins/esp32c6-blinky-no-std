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

# Make local cache directories for container to mount to
directories=(.profile/.vscode-server/extensions .profile/.vscode-server-insiders/extensions .profile/vscode/history .profile/root/history)
for dir in "${directories[@]}"; do
  mkdir -p "${local_workspace_path}/${dir}"
done

# Ensure that docker has not created folder when mounting a file that does not exist
files=(.ssh/id_rsa .ssh/config .ssh/known_hosts .gitconfig)
for file in "${files[@]}"; do
  if [ -d ~/"$file" ]; then
    rmdir -rf ~/"$file" # this is often caused by docker trying to mount a file that does not exist so it creates a folder
  fi
done

# Ensure directories exist and if running on a host then make directories on host and link to them
directories=(.kube)
for dir in "${directories[@]}"; do
  if [ ! -d ~/"$dir" ]; then # if we don't have a local directory
    if [ -z "$user_profile_path" ]; then
      mkdir -p ~/"$dir" # there is no host, so just add directory on host
    else
      if [ ! -d "${user_profile_path}/$dir/" ]; then
        mkdir -p "${user_profile_path}/$dir/" # create directory on host
      fi
      ln -s "${user_profile_path}/$dir/" ~/"$dir" # link to directory on host
    fi
  fi
done

# Setup .ssh directories
if [ ! -d ~/.ssh ]; then                                                        # if we don't have a local .ssh directory
  if [ -z "$user_profile_path" ] && [ ! -d "${user_profile_path}/.ssh/" ]; then # and we don't have a WLS profile path or there is no .ssh on that path
    mkdir -p ~/.ssh                                                             # then just create the .ssh directory locally
    chmod 700 ~/.ssh
  else                                           # we have WSL profile path, but does the .ssh directory exist in the expected windows path?
    if [ -d "${user_profile_path}/.ssh/" ]; then # it does, so lets copy the .ssh from windows
      cp -r "${user_profile_path}/.ssh/" ~/.ssh
      chmod 700 ~/.ssh   # if we have linked from windows, then we need the correct permisions on the directory
      chmod 600 ~/.ssh/* # if we have linked from windows, then we need the correct permisions on the files
    fi
  fi
fi

# If running on a host and files don't exist then link them to the host - for when file settings are the same as host
if [ -z "$user_profile_path" ]; then
  files=()
  for file in "${files[@]}"; do
    if [ ! -f ~/"$file" ] && [ -f "${user_profile_path}/$file" ]; then
      ln -s "${user_profile_path}/$file" ~/"$file"
    fi
  done
fi

# If running on a host and files don't exist then copy them over from host - for when file settings are different from the host
if [ -z "$user_profile_path" ]; then
  files=(.gitconfig)
  for file in "${files[@]}"; do
    if [ ! -f ~/"$file" ] && [ -f "${user_profile_path}/$file" ]; then
      cp -r "${user_profile_path}/$file" ~/"$file"
    fi
  done
fi

# Show warning for files that are expected to exist
files=(.ssh/id_rsa .ssh/config .ssh/known_hosts .gitconfig)
for file in "${files[@]}"; do
  if [ ! -f ~/"$file" ]; then
    echo "Warning expected file to exist: ~/$file"
  fi
done
