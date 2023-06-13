// Import the necessary modules from the near_sdk
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};

// The `#[near_bindgen]` macro is used to make the struct available for the NEAR blockchain.
// Derive `BorshDeserialize` and `BorshSerialize` for data serialization
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // Define the contract state
    password_solution: String,
}

#[near_bindgen]
impl Contract {
    // Initialization function for creating a new contract instance
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            password_solution: solution,
        }
    }

    // Function to return the stored solution
    pub fn get_solution(&self) -> String {
        self.password_solution.clone()
    }

    // Function to guess the solution and verify it
    pub fn guess_solution(&mut self, solution: String) -> bool {
        // Compute the SHA256 hash of the input solution
        let hashed_input = env::sha256(solution.as_bytes());
        // Convert the hashed solution into a hexadecimal string
        let hashed_input_hex = hex::encode(&hashed_input);
        // If the hashed input equals to the stored solution, log the success message and return true
        // Else log the failure message and return false
        if hashed_input_hex == self.password_solution {
            env::log_str("You may enter! This is the right password");
            true 
        }  else {
            env::log_str("You shall not pass. Please try again.");
            false
        }
    }
}

// Unit tests for the contract
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // Function to create a testing context with a given predecessor
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // Test the hash function
    #[test]
    fn debug_get_hash() {
        testing_env!(VMContextBuilder::new().build());
        let debug_solution = "tamago clarian";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    // Test the guess_solution function
    #[test]
    fn check_guess_solution() {
        // Initialize an AccountId and a testing context
        let clarian = AccountId::new_unchecked("clarian.testnet".to_string());
        let context = get_context(clarian);
        testing_env!(context.build());

        // Initialize the contract with the correct hashed solution
        let mut contract = Contract::new("e9242595b2be15297c7a9b0433e22c3d95a2c013b0635ec0a37a14b3f9e8146c".to_string());

        // Test with an incorrect solution
        let mut guess_result = contract.guess_solution("wrong answer".to_string());
        assert!(!guess_result, "Expectation: This is incorrect");
        assert_eq!(get_logs(), ["You shall not pass. Please try again."], "Expected a failure in logs");

        // Test with the correct solution
        guess_result = contract.guess_solution("tamago clarian".to_string());
        assert!(guess_result, "Expectation: This is the correct answer");
        assert_eq!(get_logs(), ["You shall not pass. Please try again.", "You may enter! This is the right password"], "Expected a successful log after previous failure");
    }
    // Post testing steps:
    // 1. Compile the updated smart contract
    // 2. Clean the state of our subaccount
    // 3. Deploy the wasm file
}