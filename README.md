# PredictChain Solana Program

Install Cargo (rust package manager) packages

Make sure you are on Solana devnet

```bash
solana config get
```

Should see "RPC URL: https://api.devnet.solana.com"

If not:

```bash
solana config set -u d
```

### Build the program

```bash
cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=dist/program
```

### Deploy the program to the blockchain

```bash
solana program deploy dist/program/program.so
```

### Update program id in client if there was no dist folder with saved keypair

### Todo:

- create makefile or bash script to make this easier
- split up lib.rs into entrypoint.rs, lib.rs, processor.rs, etc. [DONE]
