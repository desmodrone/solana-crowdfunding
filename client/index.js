const {
    Connection,
    Keypair,
  } = require('@solana/web3.js');
  const fs   = require('mz/fs');
  
  async function establishConnection() {
    const rpcUrl = 'http://127.0.0.1:8899';
    connection = new Connection(rpcUrl, 'confirmed');
    const version = await connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
  }
  async function createKeypairFromFile() {
    const secretKeyString = await fs.readFile("~/.config/solana/id.json", {encoding: 'utf8'});
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
  }