FROM rust:latest AS builder

RUN rustup install stable-x86_64-unknown-linux-musl

RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu
RUN apt-get install zip

ADD ./ ./sentence.io
WORKDIR /sentence.io

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_musl=gcc-x86-64-linux-gnu
ENV CC_x86_64-unknown-linux-musl=gcc-x86-64-linux-gnu

RUN cargo build --target x86_64-unknown-linux-musl --release
RUN mv /sentence.io/target/x86_64-unknown-linux-musl/release/bootstrap /bootstrap
RUN zip -j /bootstrap.zip /bootstrap

FROM scratch AS export
COPY --from=builder /bootstrap.zip /