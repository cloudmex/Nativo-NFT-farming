use crate::*;

//use std::mem::size_of;

pub type EventId = u128;
pub type TokenId = u128;
pub type StakeId = u64;

pub type Bidst = Bid;
pub type TimestampSec = u32;
pub type Timestamp = u64;


pub const MONTH_BLOCK_TIMESTAMP_IN_SECS: u64 =2629743;


/// Status of a auction.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum StakingStatus {
 
    //Stake has been started
    Running,
    //Stake has been paused by the owner
    Suspended,
    //Stake has been finished and can be claimed
    Finished,
    // If the owner withdraw before the block period ends.
    Canceled,
    //if the auction its ended and has a bid 
    Claimed,
    //The Stake doesnt exist
    NotFound,
    
}



/// Status of a auction.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum TimePeriod {
   AMonth=1,
   TwoMonths=2,
   TreeMonths=3 ,
    
}

/// Proposal for auctioning that are sent to this DAO.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct StEvent {
    /// Original nft owner.
    pub nft_owner: AccountId,
    /// Original nft contract.
    pub nft_contract: AccountId,
    /// NFT id in origin contract.
    pub nft_id: String,
    /// NFT media in origin contract.
    pub nft_media: Option<String>,
    /// Description of this auction.
    pub description: Option<String>,
    /// auction amount requested
    pub auction_base_requested: SalePriceInYoctoNear,
    /// auction amount that have to be payback to the nft owner
    pub auction_payback: SalePriceInYoctoNear,
    /// Current status of the auction
    pub status: StakingStatus,
    /// Submission time
    pub submission_time: EpochHeight,
    /// When somebody auctioned.
    pub auction_time: Option<EpochHeight>,
    /// When will the bidding end and the bidder can withdraw the NFT
    /// Also is the deadline when NFT owner can payback
    pub auction_deadline: Option<EpochHeight>,
    pub bidder_id: Option<AccountId>,

 }



 #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize,Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct StToken {
    pub st_id:String,
    /// Original nft owner.
    pub nft_owner: AccountId,
    /// Original nft contract.
    pub nft_contract: AccountId,
    /// NFT id in origin contract.
    pub nft_id: String,
    /// NFT media in origin contract.
    pub nft_media: Option<String>,
    /// Description of this auction.
    pub description: Option<String>,
    /// Current status of the auction
    pub status: StakingStatus,
    ///  time
    pub stake_time: EpochHeight,
    pub farm_start_at:Option<TimestampSec> ,
    pub blocked_until:Option<EpochHeight> ,
    
    pub reward_token:Option<AccountId>,
    pub reward_multiply:Option<u64>,
    pub reward_accumulated:Option<u64>,

 

   

 }

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    /// Id of the auction.
    pub bidder_id: AccountId,

    pub bid_amount: SalePriceInYoctoNear,
}




/// This is format of output via JSON for the auction.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StTokenOutput {
    /// Id of the auction.
    pub id: TokenId,
    #[serde(flatten)]
    pub token: StToken,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize,Debug,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Reward {
   
     // a flag to start/stop the rewards tokens
     pub is_minting_reward :bool,
     // the multiplier for each token
     pub reward_multiplier   :u128,
     // the max balance that can be rewarded for each token
     pub reward_balance_per_month : Balance,
     // the actual balance rewarded for each token
     pub balance_rewarded_per_month: Balance,
 
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Metrics {
    
    pub owner:AccountId,
    pub treasury:AccountId,
    pub contract_interest:u64,
    pub contract_fee :u64,
    
     
    pub contracts_whitelisted:u64,
    pub events_actives:u128,
    pub tokens_actives:u128,
    pub staking_events_current_ath :u128,
    pub staking_tokens_current_ath:u128,

}
/// This is format of output via JSON for the auction.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventOutput {
    /// Id of the auction.
    pub id: EventId,
    #[serde(flatten)]
    pub auction: StEvent,
}

/// This is format of output via JSON for the auction.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenOutput {
    /// Id of the auction.
    pub id: TokenId,
    #[serde(flatten)]
    pub auction: StToken,
}
/// This is format of output via JSON for the auction message.
#[derive( Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MsgInput {
    pub _type: Option<String>,
    pub reward_token:Option<AccountId>,
    pub blocked_period:Option<u64>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token ID
    pub token_id: String,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub metadata: TokenMetadata,
    //creator of the token
    pub creator_id: AccountId,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //keep track of the royalty percentages for the token in a hash map
    pub royalty: HashMap<AccountId, u32>,
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}


