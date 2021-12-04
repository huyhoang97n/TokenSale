use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::collections::UnorderedMap;
use near_sdk::{Timestamp, Duration, Gas};
use near_sdk::{Promise, PromiseResult};
use near_sdk::json_types::{WrappedBalance, WrappedDuration};
use std::convert::TryFrom;

use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};

use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PromiseOrValue};

near_sdk::setup_alloc!();

const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;
const TOKEN_NAME: &str = "SKILLS";
const TOKEN_SYMBOL: &str = "SKILLS";

const MINIMUM_DEPOSIT: Balance = 200_000_000_000_000_000_000_000;

const FOUNDER_ID: &str = "founder_skill.testnet";
const FOUNDATION_ID: &str = "foundation_skill.testnet";
const AIRDROP_ID: &str = "airdrop_skill.testnet";

const FOUNDER_PERCENT: f64 = 20 as f64;
const FOUNDATION_PERCENT: f64 = 20 as f64;
const TOKEN_FOR_SELL_PERCENT: f64 = 50 as f64;
const AIRDROP_PERCENT: f64 = 10 as f64;

const DECIMAL: u8 = 4 as u8;

const LOCK_DURATION: u64 = 31556926 as u64;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_foundation: ValidAccountId,
    token_lock_time: Duration,
    token_price: Balance,
    began_sale: bool,
    token: FungibleToken,
    deposit_map: UnorderedMap<AccountId, Balance>,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

impl Default for Contract {
    fn default() -> Self {
            let owner_id = ValidAccountId::try_from(env::predecessor_account_id().clone()).unwrap();
            let metadata = FungibleTokenMetadata {
                    spec: FT_METADATA_SPEC.to_string(),
                    name: TOKEN_NAME.to_string(),
                    symbol: TOKEN_SYMBOL.to_string(),
                    icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                    reference: None,
                    reference_hash: None,
                    decimals: DECIMAL,
                };

            let mut this = Self {
                contract_foundation: owner_id.clone(),
                token: FungibleToken::new(b"a".to_vec()),
                metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
                token_lock_time: LOCK_DURATION,
                began_sale: false,
                token_price: 0,
                deposit_map: UnorderedMap::new(b"a".to_vec())
            };

            this.token.internal_register_account(owner_id.as_ref());
            this.token.internal_deposit(owner_id.as_ref(), TOTAL_SUPPLY.into());
            return this
        }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(price: Balance) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        let owner_id = ValidAccountId::try_from(env::predecessor_account_id().clone()).unwrap();
        let metadata = FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: TOKEN_NAME.to_string(),
                symbol: TOKEN_SYMBOL.to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: DECIMAL,
            };

        let mut this = Self {
            contract_foundation: owner_id.clone(),
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
            token_lock_time: LOCK_DURATION,
            began_sale: false,
            token_price: price,
            deposit_map: UnorderedMap::new(b"a".to_vec())
        };

        this.token.internal_register_account(owner_id.as_ref());
        this.token.internal_deposit(owner_id.as_ref(), TOTAL_SUPPLY.into());
        this
    }
    
     //SET FUNC
    //TODO: Calculate token base on percent 
    fn calculate_tokens(&self, percent: f64) -> u128 {
        let _percent = percent as u128;
        let _decimal = DECIMAL as u128;
        return TOTAL_SUPPLY * _percent / 100;
    }
    //TODO: Send tokens 
    #[payable]
    pub fn send_tokens_to_core_team(&mut self) {
        self.only_owner();        

        let foundation_acc = ValidAccountId::try_from(FOUNDATION_ID).unwrap();
        let founder_acc = ValidAccountId::try_from(FOUNDER_ID).unwrap();
        let airdrop_acc = ValidAccountId::try_from(AIRDROP_ID).unwrap();

        self.token.internal_register_account(foundation_acc.as_ref());
        self.token.internal_register_account(founder_acc.as_ref());
        self.token.internal_register_account(airdrop_acc.as_ref());

        // self.token.storage_deposit(Some(foundation_acc.clone()), Some(true));
        // self.token.storage_deposit(Some(founder_acc.clone()), Some(true));
        // self.token.storage_deposit(Some(airdrop_acc.clone()), Some(true));

        self.token.ft_transfer(foundation_acc, U128::try_from(self.calculate_tokens(FOUNDATION_PERCENT)).unwrap(), None);
        self.token.ft_transfer(founder_acc, U128::try_from(self.calculate_tokens(FOUNDER_PERCENT)).unwrap(), None);
        self.token.ft_transfer(airdrop_acc, U128::try_from(self.calculate_tokens(AIRDROP_PERCENT)).unwrap(), None);
    }

    pub fn begin_sale(&mut self) {
        self.only_owner();
        self.began_sale = true;
    }

    #[payable]
    pub fn end_sale(&mut self) {
        self.only_owner();
        self.began_sale = false;

        for (acc, balance)in self.deposit_map.iter() {
            let _acc = ValidAccountId::try_from(acc).unwrap();
            self.token.internal_register_account(_acc.as_ref());
            self.token.ft_transfer(_acc, U128::try_from(balance).unwrap(), None);
        }
    }

    pub fn get_remaining_tokens(&self) -> Balance {
        let token_for_sale = self.calculate_tokens(TOKEN_FOR_SELL_PERCENT);
        let total_tokens_sold: Balance = self.deposit_map
            .values()
            .sum();

        return TOTAL_SUPPLY - token_for_sale - total_tokens_sold;
    }

    //TODO: SellTokens
    #[payable]
    pub fn buy_tokens(&mut self) {
        let deposit_amount = env::attached_deposit();

        assert!(
            deposit_amount > MINIMUM_DEPOSIT,
            "Please deposit more"
            );

        assert!(
            self.began_sale, 
            "Now is not time for sale"
            );

        let signer = env::signer_account_id();
        let token_received = self.calculate_tokens_received(deposit_amount);

        assert!(
            token_received <= self.get_remaining_tokens(),
            "Not enough tokens for sale"
            );

        self.deposit_map.insert(&signer, &token_received);
    }
    
    // #[payable]
    // pub fn widthdraw_tokens(&mut self, acc: AccountId) {
    //     self.only_owner();
    //     let amount = self.deposit_map.get(&acc).unwrap_or(0);
    //
    //     let receiver_id = ValidAccountId::try_from(acc.clone()).unwrap();
    //     self.token.internal_register_account(receiver_id.as_ref());
    //     self.token.ft_transfer(receiver_id, U128::try_from(amount).unwrap(), None);
    //     self.deposit_map.remove(&acc);
    // }

    fn calculate_tokens_received(&self, amount: Balance) -> Balance {
        let _decimal = DECIMAL as u128;
        return amount / self.token_price;
    }

    pub fn get_token_price(&self) -> Balance {
        return self.token_price;
    }

    pub fn set_token_price(&mut self, new_price: Balance) {
        self.only_owner();
        self.token_price = new_price; 
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }

    fn only_owner(&self) {
        let predecessor = env::predecessor_account_id();
        let receiver_id = ValidAccountId::try_from(predecessor.clone()).unwrap();

        assert_eq!(
            &receiver_id,
            &self.contract_foundation,
            "Only contract owner can call this fn"
            );
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

