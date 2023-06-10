FROM rust:latest AS build
WORKDIR /build
RUN rustup default nightly
COPY src src
COPY Cargo.lock .
COPY Cargo.toml .
COPY Rocket.toml .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY static static
COPY templates templates
COPY --from=build /build/target/release/starsearch .
EXPOSE 8000
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="8000"
ENTRYPOINT [ "/app/starsearch" ]