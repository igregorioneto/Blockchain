const express = require('express');
const bodyParser = require('body-parser');
const axios = require('axios');
const { Connection, PublicKey, Transaction, SystemProgram, Keypair, TransactionInstruction } = require('@solana/web3.js');

const app = express();
const port = 3000;

app.use(bodyParser.json());

const rpcUrl = 'https://api.devnet.solana.com';
const contractAddress = '';
const privateKey = '';

app.post('/vote/yes', async (req, res) => {
    const isYes = true;
    try {
        const instructionData = JSON.stringify({ vote: isYes });
        
        const connection = new Connection(rpcUrl, 'recent');

        const senderKeypair = Keypair.fromSecretKey(new Uint8Array(Buffer.from(privateKey, 'base64')));
        
        const instruction = new TransactionInstruction({
            keys: [{ pubkey: new PublicKey(contractAddress), isSigner: false, isWritable: true }],
            programId: new PublicKey(contractAddress),
            data: Buffer.from(instructionData, 'utf-8'),
        });

        const transaction = new Transaction().add(instruction);
        transaction.sign(senderKeypair);

        const signature = await connection.sendTransaction(transaction, [senderKeypair]);

        res.send(`Voto "SIM" registrado com sucesso! Transação ${signature}`);
    } catch (error) {
        console.error('Erro ao votar', error);
        res.status(500).send('Erro ao votar.');
    }
});

app.post('/vote/no', async (req, res) => {
    const isYes = false;
    try {
        const instructionData = JSON.stringify({ vote: isYes });
        
        const connection = new Connection(rpcUrl, 'recent');

        const senderKeypair = Keypair.fromSecretKey(new Uint8Array(Buffer.from(privateKey, 'base64')));
        
        const instruction = new TransactionInstruction({
            keys: [{ pubkey: new PublicKey(contractAddress), isSigner: false, isWritable: true }],
            programId: new PublicKey(contractAddress),
            data: Buffer.from(instructionData, 'utf-8'),
        });

        const transaction = new Transaction().add(instruction);
        transaction.sign(senderKeypair);

        const signature = await connection.sendTransaction(transaction, [senderKeypair]);

        res.send(`Voto "NÃO" registrado com sucesso! Transação ${signature}`);
    } catch (error) {
        console.error('Erro ao votar', error);
        res.status(500).send('Erro ao votar.');
    }
});

app.listen(port, () => {
    console.log(`Servidor rodando em http://localhost:${port}`);
});