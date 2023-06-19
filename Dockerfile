FROM rust:1-slim-bookworm AS build
WORKDIR /build
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN rustup default nightly
COPY starsearch-cli starsearch-cli
COPY starsearch-sdk starsearch-sdk
COPY starsearch-server starsearch-server
COPY Cargo.lock .
COPY Cargo.toml .
COPY Rocket.toml .
RUN cargo build -p starsearch-server --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates 
COPY static static
COPY templates templates
COPY --from=build /build/target/release/starsearch-server .
EXPOSE 8000
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="8000"
ENTRYPOINT [ "/app/starsearch-server" ]