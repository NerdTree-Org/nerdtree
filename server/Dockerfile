# only for compilation bruh :)
# run this to compile for ubuntu environment with compatible openssl(1.0.2k)
FROM ubuntu

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update -y
RUN apt install -y curl libssl-dev gcc pkg-config postgresql libpq-dev
# install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh /dev/stdin -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install diesel_cli --features postgres --no-default-features

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

VOLUME ./avatar:./avatar

EXPOSE 8080 8080

CMD ["cargo" , "run", "--release"]
