# solana-crowdfunding
Solana crowdfunding platform 


## Running the solana-crowdfunding platform locally
- start by running `solana-test-validator` on your local machine
- In your project folder, (where your Cargo.toml file is located) run `cargo build-bpf `
that will build your project and add it to your target folder.
- now run `solana program deploy <program/path/program.so>` in my case it's
`solana program deploy /target/deploy/crowdfunding.so`. Aleternatively if you can go into your deploy folder and run `Solana program deploy crowdfunding.so`.

You should get a *Program Id* similar to this: BASWjFeSUEx83XRSdjn8TASa59HUMytSnv3CfEBChYqL
if your deployment is successful.

You can confirm deployment by running `solana program show <YOUR-PROGRAM-ID-HERE>`

You should get something similar to this:
```
Program Id: BASWjFeSUEx83XRSdjn8TASa59HUMytSnv3CfEBChYqL
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: 4VWgUANTxydzbLCbfCKQ4LqpMmKThDRSXFDJx3hDzeFq
Authority: 4DfuLvEfgvStc5uokpjDQ13ZVozZdrKJTp8tXyTCCRZw
Last Deployed In Slot: 332979
Data Length: 142768 (0x22db0) bytes
Balance: 0.99486936 SOL
```