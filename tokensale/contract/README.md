# SKILLS token sale NEP141 on NEAR

```sh
cargo build --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
near dev-deploy --wasmFile res/greeter.wasm
export ID=contract_id
```

## Demo flow
```sh
near call $ID new '{"price": "20000000000000000000000", "issuer_name": "issuer"}' --accountId $CONTRACT_OWNER

near call $ID send_tokens_to_core_team '{}' --accountId $CONTRACT_OWNER

near call $ID begin_sale '{}' --accountId $CONTRACT_OWNER
```

## Login to another Account to buy tokens
```sh
near call $ID buy_tokens '{}' --accountId $CONTRACT_OWNER --deposit 1
```

