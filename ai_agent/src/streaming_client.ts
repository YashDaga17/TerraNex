import WebSocket from "ws";

export function sendIntent(url: string, intent: unknown) {
  const ws = new WebSocket(url);

  ws.on("open", () => ws.send(JSON.stringify(intent)));
  ws.on("message", (data) => {
    const raw = data.toString();
    try {
      const message = JSON.parse(raw) as { event?: string };
      if (message.event === "final_quote") {
        console.log("solver quote", raw);
        ws.close();
        return;
      }
      console.log("auction update", raw);
    } catch {
      console.log("orchestrator message", raw);
    }
  });
  ws.on("error", (err) => console.error("orchestrator error", err.message));
}
