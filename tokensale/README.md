Token sale for NEP141

# SKILLS token sale NEP141 on NEAR

```sh
cargo build --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
near dev-deploy --wasmFile res/greeter.wasm
export ID=dev-1637748940104-87011920759490
```

## Demo flow
```sh
near call $ID new '{"price": 200000000000000000}' --accountId $ID
near call $ID send_tokens_to_core_team '{}' --accountId $ID --depositYocto 1
near view $ID ft_balance_of '{"account_id": "founder_skill.testnet"}'
near view $ID ft_balance_of '{"account_id": "foundation_skill.testnet"}'
near view $ID ft_balance_of '{"account_id": "airdrop_skill.testnet"}'

near call $ID begin_sale '{}' --accountId $ID
```

## Storage deposit for account
```sh
near call dev-1637744748936-48370139139313 storae_deposit '{"account_id": "foundation_skill.testnet"}' \
  --accountId $ID --amount 0.0235
  
  near view $ID ft_balance_of '{"account_id": "'bob.$ID'"}'
  near view $ID ft_balance_of '{"account_id": "foundation_skill.testnet"}'
```
## Login to another Account to buy tokens
```sh
near call $ID buy_tokens '{}' --accountId dev_test.testnet --deposit 1
near call $ID widthdraw_tokens '{}' --accountId issuer_test.testnet --depositYocto 1

near call $ID end_sale '{}' --accountId $ID --depositYocto 1
near view $ID ft_balance_of '{"account_id": "buy_token.testnet"}'
```

g
