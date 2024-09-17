import { createRpc, Rpc } from "@lightprotocol/stateless.js";
import { Keypair } from "@solana/web3.js";

const RPC_ENDPOINT = "HELIUS-DEVNET_API_HERE";
const COMPRESSION_RPC_ENDPOINT = RPC_ENDPOINT;
export const connection: Rpc = createRpc(RPC_ENDPOINT, COMPRESSION_RPC_ENDPOINT)

export const tokenRecipient = Keypair.fromSecretKey(new Uint8Array([
]));

export const payer = Keypair.fromSecretKey(new Uint8Array([
]));

