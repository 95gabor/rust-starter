FROM rust:1.80.0-alpine3.20 AS prepare

WORKDIR /usr/src/sample

RUN apk add --no-cache musl-dev

COPY  ./Cargo.toml ./Cargo.lock ./
COPY ./apps/sample/Cargo.toml ./apps/sample/
COPY ./libs/auth/Cargo.toml ./libs/auth/

# Create a dummy main.rs to trick Cargo into resolving dependencies
RUN mkdir ./apps/sample/src ./libs/auth/src && \
    echo "fn main() { println!(\"Dummy main.rs\"); }" > ./libs/auth/src/lib.rs \
    echo "fn main() { println!(\"Dummy main.rs\"); }" > ./apps/sample/src/main.rs

RUN cargo fetch --locked --manifest-path apps/sample/Cargo.toml

COPY . .



FROM prepare AS builder

RUN cargo build --locked --release --bin sample



FROM prepare AS development

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run --bin sample"]



FROM alpine:3.20.2

COPY --from=builder /usr/src/sample/target/release/sample /usr/local/bin/sample

EXPOSE 7878

CMD ["sample"]
