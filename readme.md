# Anchor Program Setup and Deployment Guide

This guide will walk you through the process of setting up your environment to run Anchor programs on Solana, as well as building and deploying your program.

## Prerequisites

- A Unix-like operating system (Linux, macOS, WSL for Windows)
- `curl` installed on your system

## Important Note on Network and Features

As of the current date, compressed tokens and the Light Protocol are not available on the Solana mainnet. Therefore, we will be using the devnet for development and testing. Additionally, we recommend using Helius RPC for improved performance and reliability.

## Installation Steps

### 1. Install Solana

First, we'll install the Solana CLI tools:

```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.18.18/install)"
```

After running this command, follow the on-screen instructions to update your PATH.

### 2. Install Anchor

Next, we'll install Anchor version 0.29.0:

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.29.0
avm use 0.29.0
```

### 3. Initialize Solana

Now, let's initialize Solana with version 1.18.8:

```bash
solana-install init 1.18.8
```

## Verification

To verify your installations:

1. Check Solana version:

   ```bash
   solana --version
   ```

2. Check Anchor version:
   ```bash
   anchor --version
   ```

## Building and Deploying Your Program

After you've set up your Anchor project and written your program, follow these steps to build and deploy:

### 1. Build the program

To build your program with devnet features enabled, run:

```bash
anchor build -- --features devnet
```

This command compiles your program and prepares it for deployment to the devnet cluster.

### 2. Configure Solana CLI for Devnet and Helius RPC

Before deploying, set up your Solana CLI to use devnet and the Helius RPC:

```bash
solana config set --url https://devnet.helius-rpc.com/?<your-api-key>
```

Replace `<your-api-key>` with your actual Helius API key.

### 3. Deploy the program

To deploy your program to the Solana devnet, use:

```bash
anchor deploy --provider.cluster devnet
```

This command deploys your built program to the Solana devnet cluster using the Helius RPC.

## Next Steps

After successful deployment, you can:

1. Interact with your program using Anchor's TypeScript client
2. Write and run tests for your program
3. Monitor your program's performance on devnet
4. Explore the features of compressed tokens and Light Protocol on devnet

Remember that while testing on devnet with these features is possible, you'll need to plan for alternative approaches or await mainnet support for a production deployment.

For more detailed information, refer to the official [Anchor documentation](https://www.anchor-lang.com/) and stay updated with the latest developments in the Solana ecosystem.
