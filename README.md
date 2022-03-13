# Yes or no

## YesOrNo

`YesOrNo` is a smart contract in near blockchain which vote or review function and save the result in blockchain.


## Build and test

### Pre-requisites

- Rustup

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Add wasm target to your toolchain:
```shell
rustup target add wasm32-unknown-unknown
```
### Unit test
```shell
cargo test
```

### Build

```shell
sh build.sh
```

### Test in sandbox

#### Pre-requisites

- Near Sandbox Env

Follow this [tutorial](https://docs.near.org/docs/develop/contracts/sandbox) to create a sandbox in local.
It depends on `Node` and `Rustup`. 

#### Test
```shell
sh ./test/test_sandbox.sh
```

