FROM rust:1.70.0-slim-bullseye

# View app name in Cargo.toml
ARG APP_NAME=devopsvn

WORKDIR /app

COPY . .
RUN cargo build --locked --release
RUN cp ./target/release/rust-parse /bin/server

ENV ROCKET_ADDRESS=0.0.0.0
CMD ["/bin/server"]
