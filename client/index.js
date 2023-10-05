// Import necessary libraries
const fs = require('fs');
const {
    Connection,
    Keypair,
    PublicKey,
    Transaction,
    TransactionInstruction,
    SystemProgram,
    sendAndConfirmTransaction,
    clusterApiUrl,
} = require('@solana/web3.js');
const borsh = require('borsh');

// Define the structure of the data that will be stored on-chain
class CampaignDetails {
    constructor() {
        this.admin = new Uint8Array(32);  // Assuming admin is a public key represented as bytes
        this.name = '';
        this.description = '';
        this.image_link = '';
        this.amount_donated = 0n;  // Using BigInt for u64
    }
}


const CampaignSchema = new Map([
    [CampaignDetails, {
        kind: 'struct',
        fields: [
            ['admin', [32]],
            ['name', 'string'],
            ['description', 'string'],
            ['image_link', 'string'],
            ['amount_donated', 'u64'],
        ],
    }],
]);


// Program Id from your deployment
const PROGRAM_ID = new PublicKey('9F8PipnJqwqrxgej5BDixa2kTJ8Fuqf4zvBVZ3oVYFuM');

// Connection to the network
const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

// Campaign Creation
async function createCampaign(campaignDetails, payer) {
    const campaignAccount = new Keypair();
    const serializedCampaign = borsh.serialize(CampaignSchema, campaignDetails);

    const dataSize = serializedCampaign.length;

    // Determine the required balance for rent exemption
    const rentExemption = await connection.getMinimumBalanceForRentExemption(dataSize);

    // 1. Create Account and assign ownership to program
    const createAccountTx = SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: campaignAccount.publicKey,
        lamports: rentExemption,
        space: dataSize,
        programId: PROGRAM_ID,
    });

    // 2. Initialize Account with the initial state
    const initAccountTx = new TransactionInstruction({
        keys: [
            { pubkey: campaignAccount.publicKey, isSigner: true, isWritable: true },
            { pubkey: payer.publicKey, isSigner: true, isWritable: false },
        ],
        programId: PROGRAM_ID,
        data: Buffer.concat([Buffer.from([0]), serializedCampaign]),
    });

    // Bundle transactions
    const transaction = new Transaction()
        .add(createAccountTx)
        .add(initAccountTx);
    transaction.feePayer = payer.publicKey;
    transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

    const signature = await sendAndConfirmTransaction(
        connection,
        transaction,
        [payer, campaignAccount],
        {commitment: 'singleGossip', preflightCommitment: 'singleGossip'},
    );

    console.log(`Transaction ${signature} confirmed`);
    console.log(`Campaign Account: ${campaignAccount.publicKey.toBase58()}`);
}




// Main Function
async function main() {
    const keypairBuffer = fs.readFileSync("/Users/desmo/.config/solana/id.json");
    const keypairArray = JSON.parse(keypairBuffer.toString());
    const payer = Keypair.fromSecretKey(new Uint8Array(keypairArray));

    const campaignDetails = new CampaignDetails({
        admin: payer.publicKey,
        name: "Test Campaign",
        description: "This is a test campaign.",
        image_link: "http://example.com/image.png",
        amount_donated: 0
    });

    try {
        await createCampaign(campaignDetails, payer);
        console.log("Campaign created successfully!");
    } catch(err) {
        console.error(err);
    }
}

main();
