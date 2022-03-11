RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

rm -rf test_sandbox/
# shellcheck disable=SC2164
mkdir test_sandbox && cd test_sandbox

echo "create the test folder"

cp ../target/wasm32-unknown-unknown/release/contract.wasm ./