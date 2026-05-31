import WebSocket from "ws";

export function sendIntent(url: string, intent: unknown) {
  const ws = new WebSocket(url);

  ws.on("open", () => ws.send(JSON.stringify(intent)));
  ws.on("message", (data) => {
    console.log("solver quote", data.toString());
    ws.close();
  });
  ws.on("error", (err) => console.error("orchestrator error", err.message));
}
