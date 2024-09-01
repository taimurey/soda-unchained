import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SodaUnchained } from "../target/types/soda_unchained";
import { createRpc, defaultStaticAccountsStruct, defaultTestStateTreeAccounts, LightSystemProgram, Rpc } from "@lightprotocol/stateless.js";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { CompressedTokenProgram, POOL_SEED } from "@lightprotocol/compressed-token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

const AUTHORITY_SEED = Buffer.from(
  anchor.utils.bytes.utf8.encode("authority")
);

describe("soda-unchained", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const owner = anchor.Wallet.local().payer;
  const program = anchor.workspace.SodaUnchained as Program<SodaUnchained>;
  const confirmOptions = {
    skipPreflight: true,
  };

  it("initializing compressed mint and minting tokens to the owner", async () => {
    const RPC_ENDPOINT = "https://devnet.helius-rpc.com/?api-key=16ef3f61-7567-47d9-9c44-edec13422455";
    const COMPRESSION_RPC_ENDPOINT = RPC_ENDPOINT;
    const connection: Rpc = createRpc(RPC_ENDPOINT, COMPRESSION_RPC_ENDPOINT)
    const systemKeys = defaultStaticAccountsStruct();

    const mint = Keypair.generate();
    const tokenPoolPda = gettokenPoolPda(mint.publicKey, program.programId);
    const authorityPda = getAuthorityPda(mint.publicKey, program.programId);
    // Add your test here.
    const tx = program.methods.initializeServer().accounts({
      creator: owner.publicKey,
      tokenPoolPda: tokenPoolPda[0],
      mint: mint.publicKey,
      sodaAuthority: authorityPda[0],
      cpiAuthorityPda: CompressedTokenProgram.deriveCpiAuthorityPda,
      registeredProgram: systemKeys.registeredProgramPda,
      noopProgram: systemKeys.noopProgram,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      lightSystemProgram: LightSystemProgram.programId,
      compressedTokenProgram: CompressedTokenProgram.programId,
      accountCompressionAuthority: systemKeys.accountCompressionAuthority,
      merkleTree: defaultTestStateTreeAccounts().merkleTree,
      accountCompressionProgram: systemKeys.accountCompressionProgram
    })
  });
});


export function gettokenPoolPda(
  mint: PublicKey,
  programId: PublicKey
): [PublicKey, number] {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [POOL_SEED, mint.toBuffer()],
    programId
  );
  return [address, bump];
}

export function getAuthorityPda(
  mint: PublicKey,
  programId: PublicKey
): [PublicKey, number] {
  const [address, bump] = PublicKey.findProgramAddressSync(
    [AUTHORITY_SEED, mint.toBuffer()],
    programId
  );
  return [address, bump];
}