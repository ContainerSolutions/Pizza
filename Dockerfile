FROM rust:latest as builder

RUN rustup update nightly && rustup default nightly;
WORKDIR /usr/src/pizza

COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src/
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm /usr/src/pizza/target/release/pizza
COPY src src
RUN cargo build --release


FROM debian:stable-slim

COPY --from=builder /usr/src/pizza/target/release/pizza /pizza
COPY img /img
CMD /pizza
ARG PIZZA="Quattro Formaggi"
ENV PIZZA $PIZZA
EXPOSE 8000
