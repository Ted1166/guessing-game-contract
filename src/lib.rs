use std::cmp::Ordering;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen, env};
use near_rng::Rng;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Msg(String);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub guess: u64,
    // SETUP CONTRACT STATE
}
impl Default for Contract {
    fn default() -> Self {
        Self { guess: 0 }
    }
}

#[near_bindgen]
impl Contract {
    pub fn random_number(&mut self) {
        let mut rng = Rng::new(&env::random_seed());
        let value:u64 = rng.rand_range_u64(0, 100);
        self.guess = value;
    }
    pub fn get_user_guess(& mut self, number:u64) ->Result<Msg,()>{
    match self.guess.cmp(&number) {
            Ordering::Less => Ok(Msg("Too small".to_string())),
            Ordering::Greater => Ok(Msg("Too big".to_string())),
            Ordering::Equal => Ok(Msg("You win".to_string())),
    }
}
    // ADD CONTRACT METHODS HERE
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, VMContext};

    fn contract_account() -> AccountId {
        "contract".parse::<AccountId>().unwrap()
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(contract_account())
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder.build()
    }

    #[test]
    fn test_random_number() {
        let accountid = AccountId::new_unchecked("onchez.test".to_string());
        let context = get_context(accountid);
        testing_env!(context);

        let mut contract = Contract::default();
        contract.random_number();
        assert!(contract.guess > 0);
        assert!(contract.guess < 100);
    }

    #[test]
    fn test_get_user_case() {
        let accountid = AccountId::new_unchecked("onchez.test".to_string());
        let context = get_context(accountid);
        testing_env!(context);

        let mut contract = Contract::default();
        contract.random_number();
        let result = contract.get_user_guess(contract.guess);
        
        assert_eq!(result.unwrap().0, "You Win");
        //assert_eq!(result.unwrap().1, "Too Small");
        //assert_eq!(result.unwrap().2, "Too Big");
    }
}


