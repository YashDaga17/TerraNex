# TerraNex AI-Perp Exchange

Small monorepo scaffold for an agent-native perpetual exchange on Solana.

## Flow

1. User initializes the protocol vault.
2. User delegates a scoped trading session to an ephemeral key.
3. Agent signs an intent with that scoped key and sends it to the orchestrator WebSocket.
4. Orchestrator runs a short Dutch auction and chooses the best solver quote.
5. Solver submits `resolve` on-chain, bounded by session limits and oracle price checks.

## Run

This repo pins Rust `1.89.0` because the 2026 Solana dependency graph requires Rust 1.89+.

```bash
cargo fmt --all
cargo run -p nexus-orchestrator
npm --prefix ai_agent install
npm --prefix ai_agent run dev
```

Anchor build requires the Solana and Anchor toolchains plus the MagicBlock session-keys program for local session-token tests.

## Deploy

Run the orchestrator on a dedicated VPS or long-lived container runtime. Do not deploy it to Vercel serverless: the gRPC stream and WebSocket auction loop need persistent connections.

```bash
docker build -f orchestrator/Dockerfile -t nexus-orchestrator .
docker run --env-file .env -p 8080:8080 nexus-orchestrator
```
