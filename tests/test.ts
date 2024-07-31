import {
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';

import * as borsh from "borsh";
import { Buffer } from "buffer";

let PATH_TO_YOUR_SOLANA_ID_JSON = ""
let DEPLOYED_PROGRAM_ADDRESS = ""

function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
};

describe("Test to insert and get data from a calculated account created in the solana devnet:", () => {

    const connection = new Connection(`https://api.devnet.solana.com`, 'confirmed');

    const payer = createKeypairFromFile(PATH_TO_YOUR_SOLANA_ID_JSON);

    const PROGRAM_ID: PublicKey = new PublicKey(
        DEPLOYED_PROGRAM_ADDRESS
    );

    const addressNewAccount = Keypair.generate();

    it("Insert the Public Key in the calculated account", async () => {


        class Assignable {
            constructor(properties) {
                Object.keys(properties).map((key) => {
                    return (this[key] = properties[key]);
                });
            };
        };

        class AccountInfo extends Assignable {
            toBuffer() { return Buffer.from(borsh.serialize(AccountInfoSchema, this)) }

            static fromBuffer(buffer: Buffer) {
                return borsh.deserialize(AccountInfoSchema, AccountInfo, buffer);
            };
        };


        const AccountInfoSchema = new Map([
            [AccountInfo, {
                kind: 'struct',
                fields: [
                    ['info', 'string'],
                ],
            }]
        ]);

        const accountData = {
            publicKey: "***HereTheInsertedData***"
        };

        const info = JSON.stringify(accountData);

        const publicKeyInfo = new AccountInfo({
            info: info // Pasar la cadena JSON al campo 'details'
        });

        let ix = new TransactionInstruction({
            keys: [

                {
                    pubkey: addressNewAccount.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },

                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            programId: PROGRAM_ID,
            data: (
                publicKeyInfo
            ).toBuffer(),
        });
        await sendAndConfirmTransaction(
            connection,
            new Transaction().add(ix),
            [payer, addressNewAccount]
        ).then((transactionSignature) => {
            console.log("Transaction hash of insert data method:", transactionSignature);
        }).catch((error) => {
            console.error("Error sending insert data transaction:", error);
        });
    });

    it("Get the data from the calculated account", async () => {

        const publicKeyInfo = await connection.getAccountInfo(addressNewAccount.publicKey);

        if (publicKeyInfo !== null) {
            const dataAsBuffer = publicKeyInfo.data;
            const dataAsString = Buffer.from(dataAsBuffer).toString('utf-8').trim();

            console.log("Inserted data:", dataAsString);
        } else {
            throw new Error('Error getting data');
        }
    });

});     