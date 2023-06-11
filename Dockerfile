FROM rust:latest AS build
WORKDIR /build
RUN rustup default nightly
COPY starsearch-backend starsearch-backend
COPY Cargo.lock .
COPY Cargo.toml .
COPY Rocket.toml .
RUN cargo build \
    --bin starsearch-backend \
    --release

FROM debian:stable-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates
COPY static static
COPY templates templates
COPY --from=build /build/target/release/starsearch-backend starsearch
EXPOSE 8000
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="8000"
ENTRYPOINT [ "/app/starsearch" ]