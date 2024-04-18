FROM rust:1.77.2-alpine AS builder
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder ./target/release/arcanus-psw ./target/release/arcanus-psw
CMD ["/target/release/arcanus-psw"]

# Commands
#
# Build
# docker buildx build -t lpn-server:v01 .
#
# Run
# docker run -p 4200:4200 lpn-server:v01