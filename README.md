# Nostr CPU pubkey miner

A simple tool to mine nostr vanity pubkeys. Currently only supports hexadecimal vanity keys.
Tested on Windows but should run on other platforms supported by the rust compiler without any problems.
This is quick and dirty code, but works well enough.

## Compilation

After [installing rust](https://rustup.rs/), clone the repo and compile the tool :
```
git clone https://github.com/lacaulac/nostr-pubminer.git
cd nostr-cpu-pubkey-miner
cargo build --release
```

The executable will be in the newly created `target/release` directory.

## Usage

Every generated keypair will be stored in a `output.csv` file in the directory the tool is launched from.

Generating vanity keypairs with the pubkey beginning with `deadbeef` using 11 threads :

```
./vanitypubkey deadbeef 12
```

Running a one-core benchmark with 10000 iterations : 

```
./vanitypubkey benchmark 10000
```

## Architecture

A user-defined amount of threads continuously generate random keypairs and send them to the main thread. The main thread checks every pubkey and logs the ones that match the tool's arguments into the `output.csv` file.