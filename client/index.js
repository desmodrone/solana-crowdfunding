// Import necessary libraries
const {
    Connection,
    Keypair,
    PublicKey,
    Transaction,
    SystemProgram,
    TransactionInstruction,
} = require('@solana/web3.js');
const fs = require('mz/fs');
const os = require('os');
const path = require('path');

// Connection setup
const rpcUrl = 'http://127.0.0.1:8899';
const connection = new Connection(rpcUrl, 'confirmed');

// Example programId. Replace with actual deployed program's id
const programId = new PublicKey('2N9QwVW2HiDBnVpq4jFMEt3Q66w8D8udiBb7xXNVgGga');

// Establish connection and log the version
async function establishConnection() {
    const version = await connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
}


// Fetch Keypair from a local file
async function createKeypairFromFile() {
    const idPath = path.join(os.homedir(), '.config', 'solana', 'id.json');
    const secretKeyString = await fs.readFile(idPath, {encoding: 'utf8'});
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}

// Implement basic transaction setup, signing, and sending
async function setPayerAndBlockhashTransaction(instructions) {
    const transaction = new Transaction();
    instructions.forEach(instruction => transaction.add(instruction));
    transaction.feePayer = (await createKeypairFromFile()).publicKey;
    transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    return transaction;
}

async function signAndSendTransaction(transaction) {
    try {
        const keypair = await createKeypairFromFile();
        transaction.partialSign(keypair);
        const signedTransaction = transaction.serialize();
        const signature = await connection.sendRawTransaction(signedTransaction, {skipPreflight: false, preflightCommitment: 'singleGossip'});
        console.log('Signature:', signature);
    } catch (err) {
        console.error('Error in signAndSendTransaction:', err);
    }
}


async function interactWithProgram() {
    try {
        // Example public key for new account, seed should be unique per each new account
        const seed = 'UniqueSeed123';
        const newAccountPubkey = await PublicKey.createWithSeed(
            (await createKeypairFromFile()).publicKey,
            seed,
            programId,
        );

        // instruction
        const instructionData = Uint8Array.of(0);
        console.log('Sending transaction with data:', Buffer.from(instructionData)); 
        
        const instruction = new TransactionInstruction({
            keys: [{pubkey: newAccountPubkey, isSigner: false, isWritable: true}],
            programId: programId,
            data: Buffer.from(instructionData),
        });

        const transaction = await setPayerAndBlockhashTransaction([instruction]);
        await signAndSendTransaction(transaction);

    } catch (err) {
        console.error('Error in interactWithProgram:', err);
    }
}

// Run methods
establishConnection().then(() => {
    interactWithProgram().catch(err => console.error(err));
});
