# Rusty-Merkle-Tree

An implementation of [Merkle Tree](https://en.wikipedia.org/wiki/Merkle_tree) in Rust.

To run the program, use `cargo run`, and then use the commands below. Initially, the program will start with an empty tree, to which you can add elements.

## Commands

### --help
To see the available commands.
Usage: `--help`

### build
To create a Merkle Tree from a set of hashes.
Usage: `build <hash-1> <hash-2> ... <hash-n>`
Example: 
``` 
build ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb 3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d
```

### build-raw
To create a Merkle Tree from a set of raw texts.
Usage: `build-raw <raw-text-1> <raw-text-2> ... <raw-text-n>`
Example:
``` 
build-raw a b c d
```

### add-raw
To add to the current tree a raw text
Usage: `add-raw raw-text`
Example:
``` 
add-raw a
```

### add
To add to the current tree a hashed text
Usage: `add 32-bytes-hash`
Example:
``` 
add ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
```

### verify
To verify with a proof if a leaf of an index is part of the tree
Usage: `verify proof1 proof2 ... proofN seed index`
Example:
``` 
build-raw a b c
verify ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
                d50c873877f38fcbc56dbe836b9d979912efcb587ed8eea919372d403b5c2bd4 3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d, 1
```

### proof
To create a proof of a leaf at an index.
Usage: `proof index`
Example:
``` 
proof 1
```

### print
To print the tree
Usage: `print`

