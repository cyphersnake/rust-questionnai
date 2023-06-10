# Interpreter Question

Suppose we added the following bytecode instructions to our language:
- `SEND_CHANNEL`:
    Pops the channel and a value from the stack and send the value on the channel using a blocking send
- `RECV_CHANNEL`:
    Pops the channel from the stack, receives a value from the channel (this may block), and push the resulting value back onto the stack
- `SPAWN`:
    Pop two functions from the stack and spawn them as concurrent tasks 

Describe in a few sentences how each bytecode instruction could be interpreted, and how your interpreter or language runtime could deal with the blocking nature of the send and the receive instructions.

## Answer

The presence of such instructions implies the need to implement at least a context-switching mechanism between tasks for the interpreter and a logic for choosing how and which task to perform. 

- `SEND_CHANNEL` - In a concurrent programming language, this operation could potentially block if the channel is full. Interpreter would need to handle this potential blocking situation, typically by suspending the execution of the current task until the channel is ready to receive more data. This requires a concurrent or parallel runtime system that supports task suspension and resumption, and communication between tasks via channels.

- `RECV_CHANNEL` - As with `SEND_CHANNEL`, this operation could block if there's no data available in the channel to read. Interpreter needs to handle this situation, typically by suspending the current task until there's data available in the channel. Once the data is available, the task can be resumed and the received data is pushed onto the stack. The mechanism for "waking up" a task depends on the implementation of the scheduler.

- `SPAWN` - This instruction doesn't involve blocking per se, but it does involve creating and managing concurrent tasks. Interpreter would need to provide support for task creation, scheduling, and synchronization. These tasks would then run concurrently with the existing execution context, potentially interacting with it and with each other via shared memory or channels.

In a practical implementation, the runtime system might use concepts like green threads or fibers for lightweight concurrency, along with synchronization primitives like channels, locks, and condition variables to coordinate between tasks. The specific choice of techniques and mechanisms would depend on the needs and constraints of the target execution environment.

# Blockchain Questions

## 5. Explain some of the ways hashing functions enable blockchain technology

Hash functions are integral to blockchain technology and provide several key capabilities:

- Blockchain Linking: Each block also contains the hash of the previous block, forming a chain of blocks. This is what we call a blockchain. This structure ensures that changing a single block would require changing all subsequent blocks, which is computationally infeasible with large blockchains like Bitcoin.

- Immutability: Each block in a blockchain is identified by a hash, which is calculated based on the contents of the block. If any part of the block is modified, the hash will change, making it obvious that the block has been tampered with. This ensures the immutability of the blockchain, a key property for its function as a secure ledger.

- Integrity: Hashes are used to verify the integrity of data. Given a piece of data and its hash, you can quickly check if the data has been altered by calculating the hash of the data and comparing it to the original hash.

- Mining: Hash functions are also used in the process of mining new blocks (Proof of Work). Miners have to find a value that, when hashed with the data in the block, produces a hash that meets certain conditions.

## 6. Briefly explain Bitcoin's UTXO model of transaction validation (separate from POW)

Bitcoin uses the Unspent Transaction Output (UTXO) model for transaction validation. In this model, a transaction is a transfer of Bitcoin currency that is associated with a certain amount of data, specifically input and output data.

- Inputs are references to previous transactions' unspent outputs, proving that the sender has the funds required to make the transaction.
- Outputs contain the address that the Bitcoin is being sent to and the amount being sent. These outputs will be the inputs of future transactions.
- Each transaction consumes (spends) previous output(s) and creates new output(s) that can be spent by future transactions. The UTXO model is stateless, meaning only the list of UTXOs is required to validate a transaction, not the entire transaction history of each coin.

## 7. What is the structure of a Block in bitcoin and how does it relate to the 'blockchain' (merkle tree vs merkle list of merkle trees)

A block in the Bitcoin blockchain is a collection of transactions that are confirmed and then bundled together. Each block has a specific structure and contains the following key elements:

### Block
| Field               | Description                                  | Size                                    |
| --                  | --                                           | --                                      |
| Magic no            | value always 0xD9B4BEF9                      | 4 bytes                                 |
| Blocksize           | number of bytes following up to end of block | 4 bytes                                 |
| Blockheader         | consists  of 6 items                         | 80 bytes                                |
| Transaction counter | positive integer VI = VarInt                 | 1 - 9 bytes                             |
| transactions        | the (non empty) list of transactions         | <Transaction counter>-many transactions |

### Blockheader
| Field           | Purpose                                                       | Updated When...                                         | Size (Bytes)   |
| --              | --                                                            | --                                                      | --             |
| Version         | Block version number                                          | You upgrade the software and it specifies a new version | 4              |
| hashPrevBlock   | 256-bit hash of the previous block header                     | A new block comes in                                    | 32             |
| hashMerkleRoot  | 256-bit hash based on all of the transactions in the block    | A transaction is accepted                               | 32             |
| Time            | Current block timestamp as seconds since 1970-01-01T00:00 UTC | Every few seconds                                       | 4              |
| Bits            | Current target in compact format                              | The difficulty is adjusted                              | 4              |
| Nonce           | 32-bit number (starts at 0)                                   | A hash is tried (increments)                            | 4              |

The term 'blockchain' comes from the fact that these blocks are linked together in a chain. Each block refers to the one before it by including the hash of the previous block in its block header. This creates a chain of blocks from the very first block (genesis block) to the most recent block.

### 'merkle tree' vs 'merkle list of merkle trees'

A Merkle tree is a binary tree of hashes. Transactions in a Bitcoin block are put into a Merkle tree, and the root of that tree is put into the block header. This allows anyone to check if a specific transaction is included in a block without needing all the transactions.

However, the structure of the Bitcoin block is actually a list of transactions, not a Merkle tree. It's the references to the transactions in the block header that are arranged in the form of a Merkle tree.

The concept of a Merkle list of Merkle trees might refer to a data structure where each element in the list is a Merkle tree. This structure would allow the efficient verification of subsets of the data, not just individual elements. Bitcoin uses a list of transactions and a single Merkle tree in the block header, not a list of Merkle trees.

## 8. What problem/s are POW/POS trying to solve? discuss/compare (byzantine fault tolerance, reaching a single consensus on a p2p network)

Proof of Work (POW) and Proof of Stake (POS) are consensus algorithms used in blockchain networks to validate transactions and add new blocks to the blockchain.

- POW does this by solving some kind of computational problem (most often meaningless)
- POS does this through the mechanism of staking and economic incentives

Byzantine Fault Tolerance: Both POS and POW provide a solution to the Byzantine Generals Problem, a situation where distributed nodes must agree on a single strategy to avoid catastrophic system failure, but some of the nodes are unreliable. In blockchain, this translates to the need to agree on the validity of transactions and the order in which they are added to the blockchain.

Single Consensus in a P2P Network: POW and POS also ensure that all nodes in the network reach a consensus on the state of the blockchain.
