# shellcheck disable=SC2164
cd ./sandbox

# build test js
npm init
npm i near-api-js bn.js


cp ../../target/wasm32-unknown-unknown/release/contract.wasm ./

node test.js