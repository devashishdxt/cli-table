FROM rust:latest

RUN rustup component add rustfmt
RUN rustup component add clippy

RUN sed -i "s/deb http:\/\/deb.debian.org\/debian buster main/deb http:\/\/deb.debian.org\/debian buster main contrib/" /etc/apt/sources.list
RUN sed -i "s/deb http:\/\/security.debian.org\/debian-security buster\/updates main/deb http:\/\/security.debian.org\/debian-security buster\/updates main contrib/" /etc/apt/sources.list
RUN sed -i "s/deb http:\/\/deb.debian.org\/debian buster-updates main/deb http:\/\/deb.debian.org\/debian buster-updates main/" /etc/apt/sources.list

RUN apt update
RUN apt install fonts-firacode