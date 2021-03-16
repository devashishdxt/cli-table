#!/bin/bash
USERNAME=${1}
USER_UID=${2}
USER_GID=${3}

if [ ! -z "$USERNAME" ]; then
   # Create the user
   addgroup --gid $USER_GID $USERNAME
   adduser --uid $USER_UID --gid $USER_GID --disabled-password $USERNAME
fi
