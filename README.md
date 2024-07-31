# Basic Template for Deploying a Program on the Solana Blockchain

This program is designed for native deployment on the Solana blockchain. It utilizes Solana's native APIs and is written in Rust without the use of frameworks like Anchor.

 1. Install Solana-cli:
 Follow the instructions in the official Solana CLI installation guide: https://docs.solanalabs.com/es/cli/install

 2. Set solana RPC:
  ```
   solana config set --url https://api.devnet.solana.com
  ```
    
 3. Install program dependencies:
 Run the following command to install the necessary dependencies: 
  ```
   npm install
  ```
 4. Compile and Deploy the Program:
 Execute the following script to compile and deploy your program:
  ```
   ./cicd
  ```
