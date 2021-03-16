FROM rust:latest

ARG USERNAME
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

COPY scripts/*.sh /tmp/cli-table-scripts/

# Create user
RUN bash /tmp/cli-table-scripts/create-user.sh "${USERNAME}" "${USER_UID}" "${USER_GID}"

# Install rustfmt and clippy
RUN rustup component add rustfmt
RUN rustup component add clippy

# Enable contrib in debian sources
RUN sed -i "s/deb http:\/\/deb.debian.org\/debian buster main/deb http:\/\/deb.debian.org\/debian buster main contrib/" /etc/apt/sources.list
RUN sed -i "s/deb http:\/\/security.debian.org\/debian-security buster\/updates main/deb http:\/\/security.debian.org\/debian-security buster\/updates main contrib/" /etc/apt/sources.list
RUN sed -i "s/deb http:\/\/deb.debian.org\/debian buster-updates main/deb http:\/\/deb.debian.org\/debian buster-updates main/" /etc/apt/sources.list

# Update
RUN apt update

# Install FiraCode font
RUN apt install fonts-firacode

# Set user
USER ${USERNAME}
