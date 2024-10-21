# multistage docker build
FROM messense/rust-musl-cross:aarch64-musl as builder
ENV SQLX_OFFLINE=true
WORKDIR /app

# Install protoc
RUN apt-get update && apt-get install -y protobuf-compiler
# update rust toolchain
RUN rustup update

# copy relevant source code
COPY ./chat ./chat
COPY ./protos ./protos

# build and show build result
RUN cd chat && cargo build --release --target aarch64-unknown-linux-musl
RUN ls /app/chat/target/aarch64-unknown-linux-musl/release

# final stage
FROM alpine:3.20

WORKDIR /app

# Create a non-root user and group
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Set permissions for /app
RUN chown -R appuser:appgroup /app

# Switch to the non-root user
USER appuser

ARG APP_NAME
ARG APP_PORT

COPY --from=builder /app/chat/target/aarch64-unknown-linux-musl/release/$APP_NAME /app/$APP_NAME

EXPOSE $APP_PORT
