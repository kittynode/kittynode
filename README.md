# Checklist

- support current stn interface (windows, mac, linux)
- support deploying taiko w/ deployment artifacts
- later: organizes simple-taiko-node configs in ~/.stn

# Installation

- Simply compile the binary and add it to your path
- All commands accessible via `./stn`. Start with `./stn help`!

# CLI

- `stn config`: set up the configuration details of your stn, declaratively
- `stn help`: view all stn commands
- `stn start`: start up your stn
- `stn stop`: stop your stn
- `stn delete`: delete your stn
- `stn upgrade`: upgrade your stn instance
- `stn logs (proposer|prover|zkevm|client)`: view all logs

# `stnd config` flow

Hello! What would you like to config?

1. Enable proposer
2. Enable prover
3. Update L1 node RPC

## L1 node RPC

- Enter http: [prefilled with docker local]
- Enter ws: [prefilled with docker local]

### Proposer

- Enter your proposer private key:
- Enter a prover endpoint: [prefilled with local, link to market]

### Prover

- Enter your prover private key:
- Enter SGX Raiko host: [prefilled with local, link to market]

# Problems

1. simple-taiko-node doesn't include running SGX prover
2. new prover market page is needed
3. hard to understand how to run a prover in SGX mode, or ZK mode, etc.
