import bs58 from "bs58";
import { Keypair } from "@solana/web3.js";
import nacl from "tweetnacl";
import { v4 as uuid } from "uuid";

export type IntentInput = {
  owner: string;
  session: string;
  pair: string;
  side: "long" | "short";
  size: number;
  limitPrice: number;
  maxSlippageBps: number;
  expiresSlot: number;
};

export function loadSessionKeypair(): Keypair {
  const secret = process.env.SESSION_PRIVATE_KEY;
  if (!secret) return Keypair.generate();
  return Keypair.fromSecretKey(bs58.decode(secret));
}

export function signIntent(input: IntentInput, keypair: Keypair) {
  const payload = {
    id: uuid(),
    owner: input.owner,
    session: input.session,
    pair: input.pair,
    side: input.side,
    size: input.size,
    limit_price: input.limitPrice,
    max_slippage_bps: input.maxSlippageBps,
    expires_slot: input.expiresSlot
  };

  const bytes = new TextEncoder().encode(JSON.stringify(payload));
  const signature = bs58.encode(nacl.sign.detached(bytes, keypair.secretKey));
  return { ...payload, signature };
}
