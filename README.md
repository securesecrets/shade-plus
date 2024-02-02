# Secret Plus Utils

This repo contains some packages from [cw-plus](https://github.com/CosmWasm/cw-plus) that have been updated to work with the Secret Network's Cosmwasm v1.

## Differences from original repos?

Since Secret Network's `cosmwasm-std` does not support anything requiring the iterator feature, that functionality has been commented or left out. The `ContractInfo` also has an additional field for `code_hash` so this fork accounts for that difference.

## Imports

TODO
