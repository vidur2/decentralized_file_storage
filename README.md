# An Efficient File Blockchain
The purpose of this project is to create a decentralized webhosting system that is fast and scalable. Currently, other decentalized storage solutions such as IPFS and Arweave are slow, and sometimes unreliable at loading data.

## The Technology
This blockchain utilizes the basic principle of a naivechain, meaning that the data in a given block is hashed, and that hash is used to identify its position in the chain (think of a linked list). Where it moves away from the traditional format is the proofing of a block. It also deviates from a traditional blockchain in that each node also acts as a mining node, so updates to the blockchain are provided over websocket instead of HTTP Requests, making it much faster for a verified block to spread. Each node is connected to each other in a peer-to-peer structure. Additionally a single block is used to store a single file, making it faster for read operations than a traditional file blockchain.

### File Blockchain
The file blockchain  is no proof of work or stake. A file is just stored on the blockchain, and a person storing the file is charged a certain amount of cryptocurrency per gigabyte of data, and request fielded. The security is in the fact that if someone wants to spam the file blockchain, they will be charged crypto from their wallet to do so. This results in almost instantaneous file uploads.

### Gateways
This is similar to IPFS's gateway system. There is one public gateway which everyone can use located at INSERT DOMAIN NAME HERE, however, you can run your own gateway for personal use if you need faster access to the blockchain. This gateway is linked up to all the nodes that the main gateway is linked up to, and operates on a websocket protocol with the public gateway. The gateway is faster than IPFS's gateway because it essentially acts like a reverse proxy over all the backend nodes, meaning that as the number of peers increase, the speed of read operations should also increase.

## Running a Node
In order to run a node, install the binary labelled: `file_blockchain_but_efficient` and run it. Enter your credentials. If you do not have credentials SIGN UP STEPS HERE. Then connect your broadcasting ip's 8002 to your public ip's 8002. Do the same for 8003. After that you are all set! <- Currently in the works
