 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env, Gas,Balance, near_bindgen, AccountId,PromiseOrValue,PanicOnDefault,CryptoHash,serde_json::json};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::ext_contract;
use near_sdk::json_types::{U128,Base64VecU8};
use near_sdk::serde_json::{from_str};
use near_sdk::{Promise, PromiseResult};
use uint::construct_uint;
use std::collections::HashMap;


//use std::cmp::min;

//use crate::internal::*;
pub use crate::metadata::*;
pub use crate::migrate::*;
pub use crate::dao::*;

mod enumeration;
mod metadata;
mod migrate;
mod dao;
mod internal;

 
pub type EpochHeight = u64;
pub type SalePriceInYoctoNear = U128;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

 
 

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    StakeContracts,
    EventById,
    EventsByCreator,
    EventsByCreatorInner { account_id_hash: CryptoHash },
    EventsByOwner,
    EventsByOwnerInner  { account_id_hash: CryptoHash },


    TokenById,
    TokensByCreator,
    TokensByCreatorInner { account_id_hash: CryptoHash },
    TokensByOwner,
    TokensByOwnerInner  { account_id_hash: CryptoHash },

    
}




//aqui van los nombres de los metodos que mandaremos llamar
#[ext_contract(this_contract)]
trait NonFungibleToken {

    // change methods
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: String,
        msg: String,
    );

     // change methods
    fn get_promise_result(&self,contract_id:AccountId,signer_id:AccountId,msg_json:MsgInput) -> String;

}




#[ext_contract(ext_contract)]
pub trait ExternsContract {
    
    fn mint(&self, account_id:AccountId,amount: String) -> String;
    fn nft_token(& self,token_id: String);

 }

 
 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountActive {

    pub owner:String,
    pub treasury:String,
   


}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NFTStaking {
    //CONTRACT DATA
    /// Owner's account ID (it will be a DAO on phase II)
    pub owner_account_id: AccountId,
    /// Owner's account ID (it will be a DAO on phase II)
    pub treasury_account_id: AccountId,
    // Transaction interest estimated for the NFT payment
    // It is based as 10000=100%
    pub contract_interest: u64,
    /// Fee payed to Nativo stakeing 
    pub contract_fee:u64, //200=2%
    // the rewards address tokens
    pub stake_ft_contracts: UnorderedMap<AccountId, Reward>,
    // // a flag to start/stop the rewards tokens
    // pub is_minting_reward : UnorderedMap<StakeId,bool>,
    // // the multiplier for each token
    // pub reward_multiplier   : UnorderedMap<StakeId,u128>,
    // // the max balance that can be rewarded for each token
    // pub reward_balance_per_month : UnorderedMap<StakeId,Balance>,
    // // the actual balance rewarded for each token
    // pub balance_rewarded_per_month: Balance,

    //the last contract reward id
    pub last_stake_contract_id: StakeId,

    //STAKING DATA
    //Index for staking event
    pub last_staking_event_id: EventId,
    //Index for token staking 
    pub last_staking_token_id: TokenId,

    //pagination by events

    // //keeps track of the staking_event struct for a given auction ID
    pub staking_event_by_id: UnorderedMap<EventId, StEvent>,
    // //keeps track of all the staking_event IDs for a given account
    pub staking_events_by_creator: LookupMap<AccountId, UnorderedSet<EventId>>,
     //keeps track of all the staking_event IDs for a given account
    pub staking_events_by_owner: LookupMap<AccountId, UnorderedSet<EventId>>,


    //pagination by token
    //keeps track of the staki&ng_event struct for a given auction ID
     
    pub staking_token_by_id: UnorderedMap<TokenId, StToken>,
    //keeps track of all the staking_event IDs for a given account
    pub staking_tokens_by_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

  
    //how much staking events are running
    pub staking_events_active: EventId,
    //how much staking tokens are running
    pub staking_tokens_active: TokenId,
     
    //how much ATH has made by events
    pub staking_events_current_ath: EventId,
    //how much ATH has made by tokens
    pub staking_tokens_current_ath: TokenId,
    
    
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PrevNftstakingExt {
   
  
       //CONTRACT DATA
    /// Owner's account ID (it will be a DAO on phase II)
    pub owner_account_id: AccountId,
    /// Owner's account ID (it will be a DAO on phase II)
    pub treasury_account_id: AccountId,
    // Transaction interest estimated for the NFT payment
    // It is based as 10000=100%
    pub contract_interest: u64,
    /// Fee payed to Nativo stakeing 
    pub contract_fee:u64, //200=2%
    // the rewards address tokens
    pub stake_ft_contracts: UnorderedMap<AccountId, Reward>,
    // // a flag to start/stop the rewards tokens
    // pub is_minting_reward : UnorderedMap<StakeId,bool>,
    // // the multiplier for each token
    // pub reward_multiplier   : UnorderedMap<StakeId,u128>,
    // // the max balance that can be rewarded for each token
    // pub reward_balance_per_month : UnorderedMap<StakeId,Balance>,
    // // the actual balance rewarded for each token
    // pub balance_rewarded_per_month: Balance,

    //the last contract reward id
    pub last_stake_contract_id: StakeId,

    //STAKING DATA
    //Index for staking event
    pub last_staking_event_id: EventId,
    //Index for token staking 
    pub last_staking_token_id: TokenId,

    //pagination by events

    // //keeps track of the staking_event struct for a given auction ID
    pub staking_event_by_id: UnorderedMap<EventId, StEvent>,
    // //keeps track of all the staking_event IDs for a given account
    pub staking_events_by_creator: LookupMap<AccountId, UnorderedSet<EventId>>,
     //keeps track of all the staking_event IDs for a given account
    pub staking_events_by_owner: LookupMap<AccountId, UnorderedSet<EventId>>,


    //pagination by token
    //keeps track of the staking_event struct for a given auction ID
     
    pub staking_token_by_id: UnorderedMap<TokenId, StToken>,
    //keeps track of all the staking_event IDs for a given account
    pub staking_tokens_by_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

  
    //how much staking events are running
    pub staking_events_active: EventId,
    //how much staking tokens are running
    pub staking_tokens_active: TokenId,
     
    //how much ATH has made by events
    pub staking_events_current_ath: EventId,
    //how much ATH has made by tokens
    pub staking_tokens_current_ath: TokenId,
    

}


 
#[near_bindgen]
impl NFTStaking {
    //Initialize the contract
    #![allow(dead_code, unused_variables,irrefutable_let_patterns,unconditional_recursion)]
    
    #[init]
    pub fn new(
        owner_account_id: AccountId,
        treasury_account_id: AccountId,
        contract_interest: u64, //800=8%
        contract_fee: u64, //200=2%
        
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        let result= Self{
            //CONTRACT DATA
            owner_account_id,
            treasury_account_id,
            stake_ft_contracts:UnorderedMap::new(StorageKey::StakeContracts.try_to_vec().unwrap()),
            contract_interest,
            contract_fee, //200=2%
            // is_minting_ntv:true,
            // reward_multiply:3,
            // reward_balance_per_month:10000,
            // balance_rewarded_per_month:0,
            last_stake_contract_id:1,
            //STAKING DATA
            last_staking_event_id: 0,
            last_staking_token_id: 0,

            // //pagination by events
            staking_event_by_id: UnorderedMap::new(StorageKey::EventById.try_to_vec().unwrap()),
            staking_events_by_creator: LookupMap::new(StorageKey::EventsByCreator.try_to_vec().unwrap()),
            staking_events_by_owner: LookupMap::new(StorageKey::EventsByOwner.try_to_vec().unwrap()),
               // //pagination by token
            
            staking_token_by_id: UnorderedMap::new(StorageKey::TokenById.try_to_vec().unwrap()),
             staking_tokens_by_owner: LookupMap::new(StorageKey::TokensByOwner.try_to_vec().unwrap()),
            //how much staking events are running
            staking_events_active: 0,
            //how much staking tokens are running
            staking_tokens_active: 0,
            //how much ATH has made by events
            staking_events_current_ath: 0,
            //how much ATH has made by tokens
            staking_tokens_current_ath: 0,
            };
        return result;
    }

    // Receive an NFT with the method nft_transfer_call 
    // This method is called from the NFT contract
    // When transfered succesful it is saved as a new requesting for auctioning
    pub fn nft_on_transfer(&mut self,sender_id: AccountId,previous_owner_id: AccountId,token_id: String,msg: String)  -> PromiseOrValue<bool>{
        
       
         let id_e:EventId = self.last_staking_token_id.into();
         let id_t:TokenId = self.last_staking_token_id as u128;

         let contract_id = env::predecessor_account_id();
         let signer_id = env::signer_account_id();
         let msg_json: MsgInput = from_str(&msg).unwrap();
        
         
         let _tipe = msg_json.clone()._type.expect("the type is empty");
       
         if _tipe == "token".to_string()  {

         //  ext_contract_nft::
         ext_contract::ext(env::predecessor_account_id())
         .with_static_gas(Gas(100_000_000_000_000))
         .nft_token(token_id)
         .then(this_contract::ext(env::current_account_id()).with_static_gas(Gas(15_000_000_000_000))
         .get_promise_result(contract_id, signer_id, msg_json) );
       

             

         }else{
            //as event
         }
       
       
        //If for some reason the contract failed it need to returns the NFT to the orig&inal owner (true)
        return PromiseOrValue::Value(false);
    }

  
    #[payable]
    pub fn calculate_reward_nft(&mut self, st_token: u128) -> StToken{
        //use a expect and explain that the auction wasnt found
        let mut nft:StToken = self.staking_token_by_id.get(&st_token.clone()).expect("the token doesn't exist");
        let signer_id =env::signer_account_id();
        let deposit = env::attached_deposit();

        assert_eq!(signer_id==nft.nft_owner,true,"You are not the NFT owner");

        //get the variables to calculate

        let farm_start_at =nft.clone().farm_start_at.expect("nft doesnt have farm_start_at ");
        let time_elapsed=  NFTStaking::to_sec(env::block_timestamp()) - farm_start_at;
        let _reward_multiply= nft.clone().reward_multiply.expect("the nft doesnt have reward multiplier"); 
        let _reward_accumulated= time_elapsed.clone() as u64 * _reward_multiply.clone() * 1000 as u64;

        nft.reward_accumulated=Some(_reward_accumulated);

        let ms = format!("{} * {} * 1000",time_elapsed,_reward_multiply);
        env::log_str(&ms);
       //validate that the blocked period has ended
       //1-if is ended claim the rewards and the tokens
       if nft.blocked_until.unwrap()<= NFTStaking::to_sec_u64(env::block_timestamp()) {
           env::log_str("the stake period has ended");
           nft.status=StakingStatus::Finished;
       }else {
           env::log_str("the stake period doesnt has ended");
           nft.status=StakingStatus::Running;
       }


       self.staking_token_by_id.insert(&st_token, &nft.clone());
       
    
        nft
   
       }   
   
     #[payable]
     pub fn withdraw_nft_owner(&mut self, st_token: u128,accept_withdraw:Option<bool>){
         //use a expect and explain that the auction wasnt found
         let   nft:StToken = self.staking_token_by_id.get(&st_token).expect("the token doesn't exist");
         let signer_id =env::signer_account_id();
         let deposit = env::attached_deposit();
 
         assert_eq!(signer_id==nft.nft_owner,true,"You are not the NFT owner");
         assert_eq!(deposit==200000000000000000000000,true,"Please,attach 0.2"); //penalty 0.2 nears


        //validate that the blocked period has ended
        //1-if is ended claim the rewards and the tokens
        if nft.blocked_until.unwrap()<= NFTStaking::to_sec_u64(env::block_timestamp()) {
            // the time has ended
             
                let mut token_updated:StToken=    self.calculate_reward_nft(st_token);

                token_updated.status=StakingStatus::Claimed;
                self.staking_token_by_id.insert(&st_token.clone(), &token_updated.clone());
                self.staking_tokens_active-=1;

                //return the token 
                this_contract::ext(env::current_account_id())
                .with_static_gas(Gas(100_000_000_000_000))
                .nft_transfer(token_updated.clone().nft_owner, token_updated.clone().nft_id.to_string(), "NFT claimed".to_string());

                //pay the rewards
                //make a  ft_transfer_call

            //retrieve the deposit amount
            Promise::new(token_updated.clone().nft_owner).transfer(deposit);

        }else {
             
            let token_updated:StToken=    self.calculate_reward_nft(st_token);
            
            if accept_withdraw.is_some() {
                    // the user accept withdraw the nft without the rewards and pay the penalty fee
                
                //send the penaltuy to the contract treasury
                Promise::new(self.treasury_account_id.clone()).transfer(deposit);

                 //return the token 
                this_contract::ext(env::current_account_id())
                .with_static_gas(Gas(100_000_000_000_000))
                .nft_transfer(token_updated.nft_owner, token_updated.nft_id.to_string(), "NFT claimed".to_string());



            }else{
                panic!("please accept withdraw the nft without the time ended");
            }
            
        }
  
    
        }   
    
  // Método de procesamiento para promesa
  pub fn get_promise_result(&mut self ,contract_id:AccountId,signer_id:AccountId,msg_json:MsgInput)   {
         
    assert_eq!(
        env::promise_results_count(),
        1,
        "This is a callbacl module"
    );
    match env::promise_result(0) {
        PromiseResult::NotReady => unreachable!(),
        PromiseResult::Failed => {
            env::log_str( &"the external contract failed".to_string());
              
         }
        PromiseResult::Successful(result) => {
            let value = std::str::from_utf8(&result).unwrap();
          //  env::log_str("regreso al market");
          //  env::log_str(value);
            let tg: JsonToken = near_sdk::serde_json::from_str(&value).unwrap();
 
            let reward_token = msg_json.reward_token.expect("the reward id is empty");
            let _reward = self.stake_ft_contracts.get(&reward_token.clone()).expect("the reward doesnt exist");
            let blocked_period_in_months=msg_json.blocked_period.expect("the blocked period is empty");
            let release_date:u64 = MONTH_BLOCK_TIMESTAMP_IN_SECS*blocked_period_in_months;
            let id_t = self.last_staking_token_id;
            //as token
            let new_token_st=  StToken {
                st_id:id_t.clone().to_string(),
               /// Original nft owner.
                nft_owner: signer_id.clone(),
               /// Original nft contract.
                nft_contract: contract_id,
               /// NFT id in origin contract.
                nft_id: tg.token_id.to_string(),
               /// NFT media in origin contract.
                nft_media: Some(tg.metadata.media.expect("the media is empty")),
               /// Description of this auction.
                description:Some(tg.metadata.description.expect("the description is empty")),
               /// Current status of the auction
                status: StakingStatus::Running,
               /// Submission time
            
               stake_time: NFTStaking::to_sec_u64(env::block_timestamp()) ,
               farm_start_at:Some(NFTStaking::to_sec(env::block_timestamp()) ),
               blocked_until:Some(NFTStaking::to_sec_u64(env::block_timestamp()) + release_date),
               reward_token: Some(reward_token),
               reward_accumulated: Some(0),
               reward_multiply: Some(_reward.reward_multiplier.try_into().unwrap()),
           
            };
            self.staking_token_by_id.insert(&id_t.clone(), &new_token_st);
            
            self.last_staking_token_id +=1;
            self.internal_add_st_token_to_owner(&signer_id, &id_t);
            self.staking_tokens_active+=1;

      
            env::log_str(
                &json!({
                "type": "new_token_st".to_string(),
                "Id":"new_token_st_id ".to_string()+&id_t.to_string(),
                })
                    .to_string(),
            );

     
            
        }
    }
}

}
