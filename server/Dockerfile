# only for compilation bruh :)
# run this to compile for ubuntu environment with compatible openssl(1.0.2k)
FROM ubuntu:16.04

RUN apt update -y
RUN apt install -y curl libssl-dev gcc pkg-config postgresql libpq-dev
# install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh /dev/stdin -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR server
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN touch src/main.rs
RUN echo "fn main() {}" > src/main.rs

RUN cargo build --release

RUN rm -rf src/main.rs
RUN rm -rf src

COPY src src
COPY migrations .

RUN rm -rf target/release/deps/nerdtree*

RUN cargo build --release

CMD ["echo" , "Compilation is done!"]