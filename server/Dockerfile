FROM rust:1.77.2 AS builder
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder ./target/release/lpn-server ./lpn-server
COPY --from=builder ./data.json ./data.json
CMD ["/lpn-server"]

# Commands
#
# Build
# docker buildx build -t lpn-server:v01 .
#
# Run
# docker run -p 8080:8080 lpn-server:v01