# Setup

### Zombienet binary:

Download the binary from [Zombienet Releases](https://github.com/paritytech/zombienet/releases).

Copy it to `./bin`.

```shell
wget https://github.com/paritytech/zombienet/releases/download/v1.3.18/zombienet-linux -P ./bin
chmod +x ./bin/zombienet-linux
```

### Relay Chain Node Binary:

Download the binary from [Polkadot Releases](https://github.com/paritytech/polkadot/releases).

Copy it to `./bin`.

```shell
wget https://github.com/paritytech/polkadot/releases/download/v0.9.32/polkadot -P ./bin
chmod +x ./bin/polkadot
```

### Parachain node Binary:

Build this repo and copy its binary node to `./bin`.

Copy it to `./bin`.

```shell
cd ..
cargo build --release
cp ./target/release/parachain-template-node zombienet/bin
cd zombienet
```

### Spawning the Network:

- Check your `./bin` content, it should have something like:

```shell
ls -shla ./bin
total 421M
4,0K drwxrwxr-x 2 . . 4,0K dez  4 23:10 .
4,0K drwxrwxr-x 3 . . 4,0K dez  4 23:06 ..
146M -rwxrwxr-x 1 . . 151M dez  2 21:10 parachain-template-node
114M -rwxrwxr-x 1 . . 114M nov  8 21:36 polkadot
162M -rwxrwxr-x 1 . . 162M nov 18 17:53 zombienet-linux
```

- Run `zombienet` tool

```shell
./bin/zombienet-linux -p native spawn config.toml
```

- After it boots up, open PolkadotJS UI in a browser:

[Relay Chain](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)

[Parachain Chain](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9955#/explorer)
