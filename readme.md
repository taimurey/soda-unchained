# Anchor Program Setup and Deployment Guide

This guide will walk you through the process of setting up your environment to run Anchor programs on Solana, as well as building and deploying your program.

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

To deploy thr program to the Solana devnet, use:

```bash
anchor deploy --provider.cluster devnet
```
