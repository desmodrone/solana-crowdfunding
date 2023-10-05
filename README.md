# solana-crowdfunding
Solana crowdfunding platform 


## Running the solana-crowdfunding platform locally
- start by running `solnana-test-validator` on your local machine
- In your project folder, (where your Cargo.toml file is located) run `cargo build-bpf `
that will build your project and add it to your target folder.
- now run `solana program deploy <program/path/program.so>` in my case it's
`solana program deploy /target/deploy/crowdfunding.so`. Aleternatively if you can go into your deploy folder and run `Solana program deploy crowdfunding.so`.

You should get a Program Id: BASWjFeSUEx83XRSdjn8TASa59HUMytSnv3CfEBChYqL
if your deployment is successful.