FROM rust:latest

RUN rustup update nightly && rustup default nightly;
WORKDIR /usr/src/pizza
COPY . .
RUN cargo build

EXPOSE 8000
CMD /usr/src/pizza/target/debug/pizza

