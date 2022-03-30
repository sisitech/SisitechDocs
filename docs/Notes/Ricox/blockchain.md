# BLOCKCHAIN TECHNOLOGY

A *block* is a growing list of encrypted records that are encrypted through cryptographic hashing.

It contains:
 - the cryptographic hash of the previous block
 - Timestamp
 - Transactional data 

A *chain* is several blocks combined in chronological order. It is irreversable in order since each additional block strengthens the verification of the previous block and hence the entire blockchain.

The blockchain database structure uses Distributed Ledger Technology which makes use of decentralization hence the database is decentralized.
A blockchain is essentially a shared immutable digital ledger of transactions that is duplicated and distributed across the entire network of computer systems on the blockchain. 

It facilitates the process of recording transactions and tracking assets in a business network.

Every time a new transaction occurs on the blockchain, a record of that transaction is added to every participant’s ledger.


## **History**

Cryptographer David Chaum first proposed a blockchain-like protocol in his 1982 dissertation "Computer Systems Established, Maintained, and Trusted by Mutually Suspicious Groups."
Further work on a cryptographically secured chain of blocks was described in 1991 by Stuart Haber and W. Scott Stornetta. They wanted to implement a system wherein document timestamps could not be tampered with. 

In 1992, Haber, Stornetta, and Dave Bayer incorporated *Merkle trees* into the design, which improved its efficiency by allowing several document certificates to be collected into one block. Under their company Surety, their document certificate hashes have been published in The New York Times every week since 1995.

The first decentralized blockchain was conceptualized by a person (or group of people) known as Satoshi Nakamoto in 2008. 

Nakamoto improved the design in an important way using a Hashcash-like method to timestamp blocks without requiring them to be signed by a trusted party and introducing a difficulty parameter to stabilize the rate at which blocks are added to the chain.
 The design was implemented the following year by Nakamoto as a core component of the cryptocurrency bitcoin, where it serves as the public ledger for all transactions on the network.

### *Block time*

The block time is the normal time it takes for the network to create an additional one block in the blockchain. Some blockchains make another block as habitually as each five seconds. By the time of block fruition(completion), the included data becomes verifiable. In digital money, this is essentially when the exchange happens, so a more limited block time implies quicker exchanges. The block time for Ethereum is set to somewhere in the range of 14 and 15 seconds, while for bitcoin it is on normal 10 minutes.

## Properties of blockchain

![blockchain properties](https://www.euromoney.com/learning/~/media/0E855B86EDF04F3C8EABAFC42917C8C6.png?la=en&hash=B67568B63CBB2C0C7311DE742F5B9E48E86DC8B9)


## **Benefits of blockchain**

1. ### *Greater trust*

With blockchain, as a member of a members-only network, you can rest assured that you are receiving accurate and timely data, and that your confidential blockchain records will be shared only with network members to whom you have specifically granted access.

2. ### *Greater security*

Consensus on data accuracy is required from all network members, and all validated transactions are immutable because they are recorded permanently. No one, not even a system administrator, can delete a transaction.

3. ### *More efficiencies*

With a distributed ledger that is shared among members of a network, time-wasting record reconciliations are eliminated. And to speed transactions, a set of rules — called a smart contract — can be stored on the blockchain and executed automatically.

### **Uses of blockchain technology**


Blockchain technology can be integrated into multiple areas. The primary use of blockchains is as a distributed ledger for cryptocurrencies such as bitcoin.
Bitcoin's transactions are recorded on a publicly viewable blockchain.

- ### *Cryptocurrencies*

Most cryptocurrencies use blockchain technology to record transactions. For example, the bitcoin network and Ethereum network are both based on blockchain.

- ### *Smart Contracts*
Blockchain-based smart contracts are proposed agreements that can be to some degree or completely executed or upheld without human interaction. One of the principle targets of a shrewd agreement is automated escrow. A critical component of shrewd agreements is that they needn't bother with a confided in outsider (like a legal administrator) to go about as a middle person between contracting elements - the blockchain network executes the agreement all alone.

- ### *Real estate*
Real estate transactions require a ton of paperwork to verify financial information and ownership and then transfer deeds and titles to new owners. Using blockchain technology to record real estate transactions can provide a more secure and accessible means of verifying and transferring ownership. That can speed up transactions, reduce paperwork, and save money.

- ### *Artist royalties*
Using blockchain technology to track music and film files distributed over the internet can make sure that artists are paid for their work. Since blockchain technology was invented to ensure the same file doesn't exist in more than one place, it can be used to help reduce piracy. What's more, using a blockchain to track playbacks on streaming services and a smart contract to distribute payments can provide greater transparency and the assurance that artists receive the money they're owed.

For more uses check out https://www.fool.com/investing/stock-market/market-sectors/financials/blockchain-stocks/blockchain-applications/ .

## **Types of blockchain networks**
 - Public blockchain networks
 - Private blockchain networks
 - Permissioned blockchain networks
 - Consortium blockchains
 - Hybrid blockchains
 - Sidechains


1. ### *Public blockchain networks*

A public blockchain is one that anybody can join and partake in, like Bitcoin. Disadvantages could incorporate significant computational power required, almost no protection for exchanges, and feeble security. These are significant contemplations for big business use instances of blockchain.

2. ### *Private blockchain networks*

A private blockchain network, like a public blockchain network, is a decentralized distributed network. Notwithstanding, one association administers the organization, controlling who is permitted to partake, execute an agreement convention and keep up with the common record. Contingent upon the utilization case, this can fundamentally support trust and certainty between members. A private blockchain can be run behind a corporate firewall and even be facilitated on premises.

3. ### *Permissioned blockchain networks*

Organizations who set up a private blockchain will commonly set up a permissioned blockchain network. It is essential to take note of that public blockchain organizations can likewise be permissioned. This puts limitations on who is permitted to take part in the organization and in what the future held. Members need to acquire a greeting or consent to join.

4. ### *Consortium blockchains*

Different associations can share the obligations of keeping a blockchain. These pre-chosen associations figure out who might submit exchanges or access the information. A consortium blockchain is great for business when all members should be permissioned and have a common obligation regarding the blockchain.

5. ### *Hybrid blockchains*
A hybrid blockchain has a combination of centralized and decentralized features. The exact workings of the chain can vary based on which portions of centralization and decentralization are used.

6. ### *Sidechains*
A sidechain is a designation for a blockchain ledger that runs in parallel to a primary blockchain. Entries from the primary blockchain (where said entries typically represent digital assets) can be linked to and from the sidechain; this allows the sidechain to otherwise operate independently of the primary blockchain.


## **Blockchain Security**

 ### Merkle tree
 This is a tree in which every "leaf" (node) is labelled with the cryptographic hash of a data block, and every node that is not a leaf (called a branch, inner node, or inode) is labelled with the cryptographic hash of the labels of its child nodes. It allows efficient and secure verification of the contents of a large data structure.

 If the hashing process is repeated with exactly the same transactions, exactly the same hashes will be created. This allows anyone using the blockchain to check that the data has not been tampered with, because ANY change in any part of the data will result in a completely different hash, affecting every iteration of hashes all the way to the root.

 ![Merkle tree](https://www.euromoney.com/learning/~/media/DA5687FC391846DFA177FD723E9C20C2.png?h=622&w=1000&hash=2C67124278CA337EC7763A545A9E0500A05CDA2B&hash=2C67124278CA337EC7763A545A9E0500A05CDA2B&la=en)



Merkle Trees serve the purpose of significantly reducing the amount of data required to be stored and transmitted or broadcast over the network by summarising sets of hashed transactions into a single root hash. As each transaction is hashed, then combined and hashed again, the final root hash will still be a standard size. 



## References
- https://en.wikipedia.org/wiki/Blockchain
- https://builtin.com/blockchain
- https://www.euromoney.com/learning/blockchain-explained/what-is-blockchain
- https://www.ibm.com/topics/what-is-blockchain