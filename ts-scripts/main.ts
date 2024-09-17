import { ComputeBudgetProgram, PublicKey, sendAndConfirmTransaction, Transaction, TransactionMessage } from "@solana/web3.js";

export const LAMPORTS_PER_SOL = 1000000000;

import {
    LightSystemProgram,
    Rpc,
    bn,
    buildAndSignTx,
    confirmTx,
    createRpc,
    defaultStaticAccountsStruct,
    defaultTestStateTreeAccounts,
    sendAndConfirmTx,
} from "@lightprotocol/stateless.js";
import { CompressedTokenProgram, createMint, mintTo, selectMinCompressedTokenAccountsForTransfer, transfer } from "@lightprotocol/compressed-token";
import { Keypair } from "@solana/web3.js";
import { createAssociatedTokenAccount, createAssociatedTokenAccountIdempotent } from "@solana/spl-token";
import BN from "bn.js";
import { connection, payer, tokenRecipient } from "./config";
import { balances, burnAllMintsInAccount } from "./balances";


const main = async () => {
    const mintKeypair = Keypair.generate();

    //keypairs
    console.log(`Payer: ${payer.publicKey.toBase58()}`);
    console.log(`Token recipient: ${tokenRecipient.publicKey.toBase58()}`);
    // /// Airdrop lamports to pay fees
    // await confirmTx(
    //     connection,
    //     await connection.requestAirdrop(payer.publicKey, 10e9)
    // );

    // await confirmTx(
    //     connection,
    //     await connection.requestAirdrop(tokenRecipient.publicKey, 1e6)
    // );

    console.log("Mint: ", mintKeypair.publicKey.toBase58())

    /// Create a compressed token mint
    const { mint, transactionSignature } = await createMint(
        connection,
        payer,
        payer.publicKey,
        9, // Number of decimals
        mintKeypair,
    );



    console.log(`create-mint  success! txId: ${transactionSignature}`);

    const systemKeys = defaultStaticAccountsStruct();


    /// Mint compressed tokens to the payer's account
    const mintToTxId = await mintTo(
        connection,
        payer,
        mint,
        payer.publicKey, // Destination
        payer,
        1e9 // Amount
    );

    console.log(`Minted 1e9 tokens to ${payer.publicKey} was a success!`);
    console.log(`txId: ${mintToTxId}`);

    /// Transfer compressed tokens
    const transferTxId = await transfer(
        connection,
        payer,
        mint,
        7e8, // Amount
        payer, // Owner
        tokenRecipient.publicKey // To address
    );

    console.log(`Transfer of 7e8 ${mint} to ${tokenRecipient.publicKey} was a success!`);
    console.log(`txId: ${transferTxId}`);


    compression(mintKeypair);
};

const compression = async (mintKeypair: Keypair) => {


    console.log(`Payer: ${payer.publicKey.toBase58()}`);
    console.log(`Token recipient: ${tokenRecipient.publicKey.toBase58()}`);


    // 1. Fetch signatures for the user
    const compressedTokenAccounts =
        await connection.getCompressedTokenAccountsByOwner(payer.publicKey, {
            mint: mintKeypair.publicKey,
        });

    const tokenAccountsArray = compressedTokenAccounts.items;

    // 2. Select accounts to transfer from based on the transfer amount
    const amount = bn(1e8);
    const [inputAccounts] = selectMinCompressedTokenAccountsForTransfer(
        tokenAccountsArray,
        amount,
    );

    // 3. Fetch recent validity proof
    const proof = await connection.getValidityProof(
        inputAccounts.map(account => bn(account.compressedAccount.hash)),
    );

    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
        units: 1000000
    });

    const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: 1
    });

    // 4. Create transfer instruction
    const ix = await CompressedTokenProgram.transfer({
        payer: payer.publicKey,
        inputCompressedTokenAccounts: inputAccounts,
        toAddress: tokenRecipient.publicKey,
        amount,
        recentInputStateRootIndices: proof.rootIndices,
        recentValidityProof: proof.compressedProof,
    });

    console.log(ix);


    const txn = new Transaction()
        .add(modifyComputeUnits)
        .add(addPriorityFee)
        .add(ix);

    const hash = await sendAndConfirmTransaction(connection, txn, [payer, tokenRecipient]);

    console.log(hash)


    const ata = await createAssociatedTokenAccountIdempotent(
        connection,
        payer,
        mintKeypair.publicKey,
        payer.publicKey,
    );


    // 4. Create the decompress instruction
    const decompressIx = await CompressedTokenProgram.decompress({
        payer: payer.publicKey,
        inputCompressedTokenAccounts: inputAccounts,
        toAddress: ata,
        amount,
        recentInputStateRootIndices: proof.rootIndices,
        recentValidityProof: proof.compressedProof,
    });

    // 5. Create the compress instruction
    const compressIx = await CompressedTokenProgram.compress({
        payer: payer.publicKey,
        owner: payer.publicKey,
        source: ata,
        toAddress: payer.publicKey,
        amount,
        mint: mintKeypair.publicKey,
    });


    const deCompresstxn = new Transaction()
        .add(modifyComputeUnits)
        .add(addPriorityFee)
        .add(decompressIx);

    const txnhash1 = await sendAndConfirmTransaction(connection, deCompresstxn, [payer]);

    console.log("decompress txn: ", txnhash1)

    const Compresstxn = new Transaction()
        .add(modifyComputeUnits)
        .add(addPriorityFee)
        .add(compressIx);

    const txnhash2 = await sendAndConfirmTransaction(connection, Compresstxn, [payer]);

    console.log("compress txn: ", txnhash2)

};


main();
