# Deployment

Use a VPS, bare-metal box, or container platform with long-lived processes.

Good targets:

- Docker Compose on a VPS
- Fly.io machines
- Railway persistent service
- Kubernetes deployment

Avoid serverless functions for the orchestrator. Yellowstone gRPC subscriptions and WebSocket intent auctions are persistent streams, so serverless timeouts will break the core execution path.

```bash
docker compose -f deploy/docker-compose.yml up -d --build
```
