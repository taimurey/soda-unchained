import { Connection, PublicKey, sendAndConfirmTransaction, Transaction, TransactionInstruction } from "@solana/web3.js";
export const LAMPORTS_PER_SOL = 1000000000;
import {
    Rpc,
    createRpc,
} from "@lightprotocol/stateless.js";
import { Keypair } from "@solana/web3.js";
import BN from "bn.js";
import { connection, payer, tokenRecipient } from "./config";
import { createBurnCheckedInstruction, getAccount, getAssociatedTokenAddress } from "@solana/spl-token";



interface WithCursor<T> {
    items: T[];
    cursor: string | null;
}

interface TokenBalance {
    balance: BN;
    mint: PublicKey;
}

function convertBalance(balanceBN: BN, decimals = 9): number {
    return balanceBN.toNumber() / Math.pow(10, decimals);
}

function processBalances(balancesData: WithCursor<TokenBalance>): { mint: string; balance: number }[] {
    return balancesData.items.map(item => ({
        mint: item.mint.toString(),
        balance: convertBalance(item.balance)
    }));
}

export async function balances() {
    const payerBalances: WithCursor<TokenBalance> =
        await connection.getCompressedTokenBalancesByOwner(payer.publicKey);
    const readablePayerBalances = processBalances(payerBalances);
    console.log("Payer Balances:", readablePayerBalances);

    const recipientBalances: WithCursor<TokenBalance> =
        await connection.getCompressedTokenBalancesByOwner(tokenRecipient.publicKey);
    const readableRecipientBalances = processBalances(recipientBalances);
    console.log("Recipient Balances:", readableRecipientBalances);

    const signatures =
        await connection.getCompressionSignaturesForOwner(payer.publicKey);
    console.log("Signatures: ", signatures);
}




async function burnAllMints(
    tokenAccounts: TokenBalance[]
) {
    const instructions: TransactionInstruction[] = [];

    for (const tokenAccount of tokenAccounts) {
        const { mint, balance } = tokenAccount;

        if (balance.isZero()) {
            console.log(`Skipping ${mint.toString()} as balance is zero`);
            continue;
        }

        try {
            const associatedTokenAddress = await getAssociatedTokenAddress(
                mint,
                payer.publicKey
            );

            // Fetch the latest account info
            const accountInfo = await getAccount(connection, associatedTokenAddress);

            // Use the actual balance from the account info
            const actualBalance = new BN(accountInfo.amount.toString());

            if (actualBalance.isZero()) {
                console.log(`Skipping ${mint.toString()} as actual balance is zero`);
                continue;
            }

            const burnInstruction = createBurnCheckedInstruction(
                associatedTokenAddress,
                mint,
                payer.publicKey,
                actualBalance.toNumber(),
                9 // Use the actual number of decimals
            );

            instructions.push(burnInstruction);
        } catch (error) {
            console.error(`Error processing mint ${mint.toString()}:`, error);
        }
    }

    if (instructions.length === 0) {
        console.log("No tokens to burn");
        return;
    }

    const transaction = new Transaction().add(...instructions);



    try {
        // Simulate the transaction first
        const simulation = await connection.simulateTransaction(transaction, [payer]);
        if (simulation.value.err) {
            console.error("Transaction simulation failed:", simulation.value.logs);
            return;
        }

        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [payer]
        );
        console.log(`Burn transaction successful. Signature: ${signature}`);
    } catch (error) {
        if (error instanceof Error && 'logs' in error) {
            console.error("Error burning tokens. Logs:", (error as any).logs);
        } else {
            console.error("Error burning tokens:", error);
        }
    }
}

// Usage
export async function burnAllMintsInAccount() {
    const tokenAccounts = await connection.getCompressedTokenBalancesByOwner(payer.publicKey);
    await burnAllMints(tokenAccounts.items);
}

balances();