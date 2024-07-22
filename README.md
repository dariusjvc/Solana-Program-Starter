# Basic Template for Deploying a Program on the Solana Blockchain

 1. Install Solana-cli:
 Follow the instructions in the official Solana CLI installation guide: https://docs.solanalabs.com/es/cli/install

 2. Set solana RPC:
  ```
   solana config set --url https://api.devnet.solana.com
  ```
    
 3. Install program dependencies:
 Run the following command to install the necessary dependencies: 
  ```
   yarn install
  ```
 4. Compile and Deploy the Program:
 Execute the following script to compile and deploy your program:
  ```
   ./cicd
  ```
