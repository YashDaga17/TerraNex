import { KeypairWallet, SolanaAgentKit } from "solana-agent-kit";
import { loadSessionKeypair, signIntent } from "./tools/intent_signer";
import { sendIntent } from "./streaming_client";

const rpcUrl = process.env.RPC_URL ?? "https://api.devnet.solana.com";
const orchestratorUrl = process.env.ORCHESTRATOR_WS ?? "ws://127.0.0.1:8080/ws/intents";
const sessionKeypair = loadSessionKeypair();

const wallet = new KeypairWallet(sessionKeypair, rpcUrl);
const agent = new SolanaAgentKit(wallet, rpcUrl, {
  OPENAI_API_KEY: process.env.OPENAI_API_KEY ?? ""
});

void agent;

const intent = signIntent(
  {
    owner: process.env.USER_AUTHORITY ?? sessionKeypair.publicKey.toBase58(),
    session: process.env.TRADING_SESSION ?? "pending-session-pda",
    pair: process.env.PAIR ?? "SOL-PERP",
    side: "long",
    size: Number(process.env.INTENT_SIZE ?? 1),
    limitPrice: Number(process.env.LIMIT_PRICE ?? 150_000_000),
    maxSlippageBps: Number(process.env.MAX_SLIPPAGE_BPS ?? 50),
    expiresSlot: Number(process.env.EXPIRES_SLOT ?? 999_999_999)
  },
  sessionKeypair
);

sendIntent(orchestratorUrl, intent);
