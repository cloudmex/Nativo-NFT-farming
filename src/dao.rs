  
 
use crate::*;


#[near_bindgen]
impl NFTStaking {

   /**/
    // set a new owner
   
    pub fn set_new_owner(&mut self,new_owner:AccountId) -> String {
        self.is_the_owner();
        self.owner_account_id=new_owner;
        self.owner_account_id.to_string()
    }
    // set a new treasury
     pub fn set_new_treasury(&mut self,new_treasury:AccountId) -> String {
        self.is_the_owner();
        self.treasury_account_id=new_treasury;
        self.treasury_account_id.to_string()
    }

    pub fn set_ft_reward_info_contract(&mut self,stake_ft_contract:AccountId,init_state:bool,_reward_multiplier:u128,_max_balance_per_month:u128) -> String {
        self.is_the_owner();
        let last_stake_id=self.last_stake_contract_id;
        let exit_stake_contract:Option<Reward>=self.stake_ft_contracts.get(&stake_ft_contract.clone());

        if exit_stake_contract.is_some() {
          //if exist update
              let new_reward_contract:Reward= Reward {
      
                // a flag to start/stop the rewards tokens
                  is_minting_reward :init_state,
                // the multiplier for each token
                  reward_multiplier   :_reward_multiplier,
                // the max balance that can be rewarded for each token
                  reward_balance_per_month : _max_balance_per_month,
                // the actual balance rewarded for each token
                  balance_rewarded_per_month: exit_stake_contract.unwrap().balance_rewarded_per_month,
            
              };
               self.stake_ft_contracts.insert(&stake_ft_contract.clone(), &new_reward_contract.clone());
                let s = format!("{} was updated",stake_ft_contract.to_string() );

                return s;
        }
        else{
          //if not create a new one
                let new_reward_contract:Reward= Reward {
        
                  // a flag to start/stop the rewards tokens
                    is_minting_reward :init_state,
                  // the multiplier for each token
                    reward_multiplier   :_reward_multiplier,
                  // the max balance that can be rewarded for each token
                    reward_balance_per_month : _max_balance_per_month,
                  // the actual balance rewarded for each token
                    balance_rewarded_per_month: 0,
              
            };
        self.stake_ft_contracts.insert(&stake_ft_contract.clone(), &new_reward_contract.clone());
        self.last_stake_contract_id+=1;

        let s = format!("the Id for {} is {}",stake_ft_contract.to_string(),last_stake_id.to_string()  );

        s
        }

 
        
    }
    
    pub fn get_stake_ft_contract(&self,stake_ft_contract:AccountId) -> Reward {
          
        self.stake_ft_contracts.get(&stake_ft_contract).expect("Contract id not found")
    }
     // set a new contract interest

     pub fn set_new_contract_interest(&mut self,new_contract_interest:u64) -> String {
         self.is_the_owner();
         self.contract_interest=new_contract_interest;
         self.contract_interest.to_string()
     }

    //   // set a new contract interest
    //    pub fn set_new_payment_period(&mut self,new_payment_period:u64) -> String {
    //       self.is_the_owner();
    //       self.payment_period=new_payment_period;
    //       self.payment_period.to_string()
    //   }
        // set a new contract interest
       pub fn set_new_contract_fee(&mut self,new_contract_fee:u64) -> String {
          self.is_the_owner();
          self.contract_fee=new_contract_fee;
          self.contract_fee.to_string()
      }

       pub fn set_is_minting_ntv(&mut self,stake_contract_id:AccountId,is_enable:bool) -> String {
          self.is_the_owner();

          let mut reward_stale:Reward=self.stake_ft_contracts.get(&stake_contract_id.clone()).expect("The contract id was not found");

          reward_stale.is_minting_reward=is_enable;
          self.stake_ft_contracts.insert(&stake_contract_id.clone(), &reward_stale);
 

        let s = format!("the status for {} is {}",stake_contract_id.to_string(), reward_stale.is_minting_reward.to_string()  );

        s
      }
      pub fn get_auctions_stats(&self) -> Metrics {
         let metrics = Metrics {
             
             
          
            owner: self.owner_account_id.clone(),
            treasury: self.treasury_account_id.clone(),
            contract_interest: self.contract_interest,
            contract_fee: self.contract_fee,
             
            contracts_whitelisted: self.last_stake_contract_id,
            events_actives: self.staking_events_active,
            tokens_actives: self.staking_events_active,
            staking_events_current_ath: self.staking_events_current_ath,
            staking_tokens_current_ath: self.staking_tokens_current_ath,
         
        };
        metrics
    }
     //method to test the remote upgrade
     pub fn market_accounts(&self) -> AccountActive {
         

        AccountActive {
              owner:self.owner_account_id.to_string(),
              treasury:self.treasury_account_id.to_string(),
               
        }
     }
 


     //validate if the owner is the caller
     #[private]
    pub fn is_the_owner(&self)   {
        //validate that only the owner contract add new contract address
        assert_eq!(
            self.owner_account_id==env::predecessor_account_id(),
            true,
            "!you are not the contract owner address¡"
        );
    }

     
   
}

 
