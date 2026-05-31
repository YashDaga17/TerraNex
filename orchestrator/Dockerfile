FROM rust:1-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY orchestrator orchestrator
COPY programs/nexus_core programs/nexus_core
RUN cargo build --release -p nexus-orchestrator

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/nexus-orchestrator /usr/local/bin/nexus-orchestrator
ENV RUST_LOG=nexus_orchestrator=info
EXPOSE 8080
CMD ["nexus-orchestrator"]
