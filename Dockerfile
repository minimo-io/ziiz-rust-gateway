# build stage
# will need redo @minimo_io
FROM rust:latest AS builder

WORKDIR /workspace

RUN apt-get update && apt-get install lld clang -y

COPY . .

RUN cargo build --release

# deploy stage
# Use a smaller image for the final runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends openssl ca-certificates && apt-get clean

# create workspace directory
WORKDIR /workspace

COPY static static

COPY settings settings

# copy binary and configuration files
# COPY --from=builder /workspace/target/release/app .
COPY --from=builder /workspace/target/release/ziiz-rust-gateway .

# expose port
EXPOSE 8080

ENV APP_PROFILE prod

ENV RUST_LOG info

# run the binary
# ENTRYPOINT ["./app"]
ENTRYPOINT ["./ziiz-rust-gateway"]