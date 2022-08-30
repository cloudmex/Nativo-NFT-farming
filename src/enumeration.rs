use crate::*;

#[near_bindgen]
impl NFTStaking {
    #![allow(dead_code, unused_variables,irrefutable_let_patterns,)]
    #[warn(unconditional_recursion)]

    //View the auction_id of the last auction
    pub fn get_contract_interest(&self)-> u64 {
        self.contract_interest
    }
 

    pub fn get_contract_owmer(&self)->AccountId {
        self.owner_account_id.clone()
    }
    pub fn get_contract_treasury(&self)->AccountId {
        self.treasury_account_id.clone()
    }
    // pub fn get_payment_period(&self)->u64 {
    //     self.payment_period.clone()
    // }
    
    pub fn get_contract_fee(&self)->u64 {
        self.contract_fee.clone()
    }
    
    // pub fn is_reward_enable_minting(&self,reward_id:StakeId)->bool {
    //     self.is_reward_enable_minting(reward_id)
    // }
   
    pub fn get_staking_token_by_id(&self,st_token:u128)->Option<StTokenOutput>{

        let _token:Option<StToken>= self.staking_token_by_id.get(&st_token);
        if _token.is_some() {

             Some(StTokenOutput {
            id:st_token.clone(),
            token:_token.unwrap(),
        })
        }
        else{
            None
        }
       
    }

     

    pub fn get_all_nfts_for_token(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<StTokenOutput> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        self.staking_token_by_id.keys_as_vector().iter()
        //skip to the index we specified in the start variable
        .skip(start as usize) 
        //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
        .take(limit.unwrap_or(50) as usize) 
        //we'll map the token IDs which are strings into Json Tokens
        .map(|st_token| self.get_staking_token_by_id(st_token.clone()).unwrap())
        //since we turned the keys into an iterator, we need to turn it back into a vector to return
        .collect()
      
    }
     


    
    //this method return  all the tokens with status published or bidded
    pub fn get_nfts_for_token_by_status(&self, status: StakingStatus,from_index: Option<U128>, limit: Option<u64>) -> Vec<StTokenOutput> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each token using an iterator
        self.staking_token_by_id.keys_as_vector().iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .filter_map(|_toke_id|{


                        let _token:Option<StTokenOutput> = self.get_staking_token_by_id(_toke_id);
                         
                    if _token.clone().is_some(){
                        if _token.clone().unwrap().token.status ==status {
                         return Some(_token.unwrap());
                        }
                        else {
                            None
                        }
                    }
                    else {
                        None
                    }
                }
            )
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
  

    
    

    pub fn get_tokens_by_owner_status(
        &self,
        account_id: AccountId,
        status: StakingStatus,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<StTokenOutput> {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.staking_tokens_by_owner.get(&account_id);
         
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector. 
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        tokens.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .filter_map(|_toke_id|{


                        let _token:Option<StTokenOutput> = self.get_staking_token_by_id(_toke_id);
                        
                    if _token.clone().is_some(){
                        if _token.clone().unwrap().token.status ==status {
                        return Some(_token.unwrap());
                                }
                                else {
                                    None
                                }
                            }
                            else {
                                None
                            }
                        }
            )
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    pub fn get_tokens_by_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<StTokenOutput> {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.staking_tokens_by_owner.get(&account_id);
         
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector. 
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        tokens.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.get_staking_token_by_id(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }


  
    pub(crate) fn to_nano(timestamp: TimestampSec) -> Timestamp {
        Timestamp::from(timestamp) * 10u64.pow(9)
    }
    
    pub(crate) fn to_sec(timestamp: Timestamp) -> TimestampSec {
        (timestamp / 10u64.pow(9)) as u32
    }
    pub(crate) fn to_sec_u64(timestamp: Timestamp) -> Timestamp {
        timestamp / 10u64.pow(9)
    }
}