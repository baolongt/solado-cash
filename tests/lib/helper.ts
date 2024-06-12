import fs from "fs";
import {
    Keypair,
} from "@solana/web3.js";
import path from 'path';


/*
  Load a locally stored JSON keypair file and convert it to a valid Keypair
*/
export function loadKeypairFromFile() {
    try {
        const filePath = path.join(__dirname, 'staker.json');
        console.log("Loading keypair from", filePath);
        const keyfileBytes = JSON.parse(fs.readFileSync(filePath, { encoding: "utf-8" }));
        // load the keypair from the file
        // parse the loaded secretKey into a valid keypair
        const keypair = Keypair.fromSecretKey(new Uint8Array(keyfileBytes));
        return keypair;
    } catch (err) {
        // return false;
        throw err;
    }
}
