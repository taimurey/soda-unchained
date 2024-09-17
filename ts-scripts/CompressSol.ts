import { buildAndSignTx, confirmTx, defaultTestStateTreeAccounts, LightSystemProgram, sendAndConfirmTx } from "@lightprotocol/stateless.js";
import { connection, payer, tokenRecipient } from "./config";
import { ComputeBudgetProgram } from "@solana/web3.js";

async function compressSol() {
    console.log(`Payer: ${payer.publicKey.toBase58()}`);
    console.log(`Token recipient: ${tokenRecipient.publicKey.toBase58()}`);

    await confirmTx(
        connection,
        await connection.requestAirdrop(tokenRecipient.publicKey, 10e9)
    );

    /// Fetch latest blockhash
    const { blockhash } = await connection.getLatestBlockhash();

    /// Compress lamports to self
    const ix = await LightSystemProgram.compress({
        payer: tokenRecipient.publicKey,
        toAddress: payer.publicKey,
        lamports: 1_000_000_000,
        outputStateTree: defaultTestStateTreeAccounts().merkleTree,
    });

    /// Create a VersionedTransaction and sign it
    const tx = buildAndSignTx(
        [ComputeBudgetProgram.setComputeUnitLimit({ units: 1_200_000 }), ix],
        tokenRecipient,
        blockhash,
        []
    );

    /// Confirm
    const txId = await sendAndConfirmTx(connection, tx);
    console.log("Transaction Signature:", txId);
}


compressSol();