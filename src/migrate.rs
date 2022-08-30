use crate::*;
use near_sdk::{Gas};

/// Gas for upgrading this contract on promise creation + deploying new contract.
pub const TGAS: u64 = 10_000_000_000_000;
pub const GAS_FOR_UPGRADE_SELF_DEPLOY: Gas = Gas(300_000_000_000_000);
pub const GAS_FOR_UPGRADE_REMOTE_DEPLOY: Gas = Gas(300_000_000_000_000);


#[near_bindgen]
impl NFTStaking {

    #[cfg(target_arch = "wasm32")]
    pub fn upgrade(self) {
        use near_sys as sys;
        assert!(env::predecessor_account_id() == self.owner_account_id);
        const GAS_FOR_UPGRADE: u64 = 20 * TGAS; //gas occupied by this fn
       // const BLOCKCHAIN_INTERFACE_NOT_SET_ERR: &str = "Blockchain interface not set.";
        //after upgrade we call *pub fn migrate()* on the NEW CODE
        let current_id = env::current_account_id();
        let migrate_method_name = "migrate".as_bytes().to_vec();
        let attached_gas = env::prepaid_gas() - env::used_gas() - Gas(GAS_FOR_UPGRADE);
        unsafe {
            // Load input (new contract code) into register 0
            sys::input(0);

            //prepare self-call promise
            let promise_id =
                sys::promise_batch_create(current_id.as_bytes().len() as _, current_id.as_bytes().as_ptr() as _);

            //1st action, deploy/upgrade code (takes code from register 0)
            sys::promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);

            // 2nd action, schedule a call to "migrate()".
            // Will execute on the **new code**
            sys::promise_batch_action_function_call(
                promise_id,
                migrate_method_name.len() as _,
                migrate_method_name.as_ptr() as _,
                0 as _,
                0 as _,
                0 as _,
                u64::from(attached_gas),
            );
        }
    }

/////////////////////METODO DE MIGRACIÖN
 
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
         let old_state: PrevNftstakingExt  = env::state_read().expect("failed");
        
        env::log_str("old state readed");
        Self {

            owner_account_id:old_state.owner_account_id,
            treasury_account_id:old_state.treasury_account_id,
            stake_ft_contracts:old_state.stake_ft_contracts,
            contract_interest:old_state.contract_interest,
            contract_fee:old_state.contract_fee,
            
            last_stake_contract_id:old_state.last_stake_contract_id,

            //STAKING DATA
            last_staking_event_id:old_state.last_staking_event_id,
            last_staking_token_id:old_state.last_staking_token_id,

            //pagination by events
            staking_event_by_id:old_state.staking_event_by_id,
            staking_events_by_creator:old_state.staking_events_by_creator,
            staking_events_by_owner:old_state.staking_events_by_owner,
            // //pagination by token
            staking_token_by_id:old_state.staking_token_by_id,
             staking_tokens_by_owner:old_state.staking_tokens_by_owner,
            // //how much staking events are running
            staking_events_active:old_state.staking_events_active,
            //how much staking tokens are running
            staking_tokens_active:old_state.staking_tokens_active,
            //how much ATH has made by events
            staking_events_current_ath:old_state.staking_events_current_ath,
            //how much ATH has made by tokens
            staking_tokens_current_ath:old_state.staking_tokens_current_ath,
        }
    }


   
    #[private]
    #[init(ignore_state)]
    pub fn cleanup()  -> Self {
        let old_state: PrevNftstakingExt = env::state_read().expect("failed");

        env::log_str("clean up state");
        Self {
       
           
            
            owner_account_id:old_state.owner_account_id,
            treasury_account_id:old_state.treasury_account_id,
            stake_ft_contracts:old_state.stake_ft_contracts,
            contract_interest:old_state.contract_interest,
            contract_fee:old_state.contract_fee,
            
            last_stake_contract_id:old_state.last_stake_contract_id,

            //STAKING DATA
            last_staking_event_id:0,
            last_staking_token_id:0,

            //pagination by events
             staking_event_by_id: UnorderedMap::new(StorageKey::EventById.try_to_vec().unwrap()),
             staking_events_by_creator: LookupMap::new(StorageKey::EventsByCreator.try_to_vec().unwrap()),
             staking_events_by_owner: LookupMap::new(StorageKey::EventsByOwner.try_to_vec().unwrap()),
            // //pagination by token
            staking_token_by_id: UnorderedMap::new(StorageKey::EventById.try_to_vec().unwrap()),
            staking_tokens_by_owner: LookupMap::new(StorageKey::TokensByOwner.try_to_vec().unwrap()),  //how much staking events are running
            
            staking_events_active:0,
            //how much staking tokens are running
            staking_tokens_active:0,
            //how much ATH has made by events
            staking_events_current_ath:0,
            //how much ATH has made by tokens
            staking_tokens_current_ath:0,
        }
    }

}