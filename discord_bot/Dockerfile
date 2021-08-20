# only for compilation bruh :)
# run this to compile for ubuntu environment with compatible openssl(1.0.2k)
FROM ubuntu:16.04

RUN apt update -y
RUN apt install -y curl libssl-dev gcc pkg-config
# install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh /dev/stdin -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR bot
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN touch src/main.rs
RUN echo "fn main() {}" > src/main.rs

RUN cargo build --release

RUN rm -rf src/main.rs
RUN rm -rf src

COPY src/main.rs src/main.rs

RUN rm -rf target/release/deps/discord_bot*
RUN cargo build --release

CMD ["echo", "Compilation Done!"]