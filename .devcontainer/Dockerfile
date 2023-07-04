FROM mcr.microsoft.com/devcontainers/universal:2-linux

WORKDIR /home/

COPY . .

RUN bash ./script.sh

ENV PATH="/root/.cargo/bin:$PATH"
