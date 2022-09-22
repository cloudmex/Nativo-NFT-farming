# Nativo NFT - *Staking*

# ![Logo](https://develop.dphj3ja30lftx.amplifyapp.com/static/media/LogoBlanco.30fcfa22.png)
<br>
NFT staking allows you to farm $Ntv as reward blocking your NFT by tim period without loosing the ownership of your NFT's

1. Secure your NFT in NFT staking and request a reward of $NTV tokens
2. Suscribe your NFT to events collections and get more rewards($NTV tokens + owner collection rewards).

## Prepare your env :

### Dev account

> `export CONTRACT_ID="dev-1663715186771-87155954122202" `

### Sub account

> ` export CONTRACT_ID="v1.nativo_stake.testnet"  `

### Make a devdeploy run:

> `./devdeploy.sh`

### Make a deploy run:

> `./build.sh`

### Make a migration run:

> `./migrate.sh`

## Initializing the contract

> ` near call $CONTRACT_ID new '{"owner_account_id": "dokxo.testnet","treasury_account_id": "dokxo.testnet","contract_interest": 800,"contract_fee": 200 }' --accountId dokxo.testnet`

## Views

### View who is the owner account

> `near view $CONTRACT_ID get_contract_owmer`

### View who is the treasury account

> `near view $CONTRACT_ID get_contract_treasury`

### View the contract interes

> `near view $CONTRACT_ID get_contract_interest`

### View the contract contract\_fee

> `near view $CONTRACT_ID get_contract_fee`

### View is the ntv minting is enabled

> `near view $CONTRACT_ID is_ntv_enable_minting`

### View the info contract metrics
>` near view $CONTRACT_ID get_auctions_stats   `
## Sets

### Set a new owner

> `near call $CONTRACT_ID set_new_owner '{"new_owner":"dokxo.testnet"}' --accountId v1.nativo.testnet`

### Set a new treasury

> `near call $CONTRACT_ID set_new_treasury '{"new_treasury":"dokxo.testnet"}' --accountId dokxo.testnet`

### Set a new contract interest

> `near call $CONTRACT_ID set_new_contract_interest '{"new_contract_interest":100}' --accountId dokxo.testnet`

### Set a new contract fee

> `near call $CONTRACT_ID set_new_contract_fee '{"new_contract_fee":100}' --accountId dokxo.testnet`

### set a new is nvt minting

> `near call $CONTRACT_ID set_is_minting_ntv '{"is_enable":true}' --accountId dokxo.testnet`

## whitelist a new reward token

> `near call $CONTRACT_ID set_ft_reward_info_contract '{"stake_ft_contract":"nativo_token.testnet","init_state":true,"_reward_multiplier":3,"_max_balance_per_month":10100}' --accountId dokxo.testnet `




# 1 How to create a Event and list NFT in it

### 1.1 Create a new event 

> ` near call $CONTRACT_ID create_event_for_nfts '{"event_info": {"event_type": "Nativo","event_owner": "dokxo.testnet","event_tittle": "title","event_description": "descrip","event_media": "media","nft_contract": "minterv2.nativo-minter.testnet","event_time": 1663791579,"event_start_at": 1663791579,"event_blocked_until": 1663791639,"reward_token": [ { "reward_address":"nativo_token.testnet","reward_multiplier":10000}],"reward_accumulated": [] }}' --accountId dokxo_test.testnet  `

 ### 1.2 Recover a event stake info
> ` near view $CONTRACT_ID get_staking_event_by_id '{"st_event":8}'  `

### List a NFT in the Event by id
> ` near call minterv2.nativo-minter.testnet nft_transfer_call '{"receiver_id": "v1.nativo_stake.testnet","token_id":"93", "msg": "{\"_type\": \"event\",\"st_event_id\": \"7\" }"}' --accountId dokxo_test.testnet --depositYocto 1 --gas 300000000000000 `

### Recover info about the NFT
> ` near view $CONTRACT_ID get_staking_token_by_id '{"st_token":6}'  `

### Also you can calculate how much reward have you winned(after the event has started).
> ` near call $CONTRACT_ID calculate_reward_nft '{"st_token":6}' --accountId dokxo_test.testnet `

### Claim your NFT and Event Rewards ( only the NFT's owner can claim  )
> ` near call $CONTRACT_ID withdraw_nft_owner '{"st_token":6}' --accountId dokxo_test.testnet  --depositYocto 1 --gas 300000000000000 `





# Stake a NFT

 
### stake a NFT into a event 
> `near call minterv2.nativo-minter.testnet nft_transfer_call '{"receiver_id": "dev-1663715186771-87155954122202","token_id":"93", "msg": "{\"_type\": \"token\",\"st_event_id\": \"8\" }"}' --accountId dokxo_test.testnet --depositYocto 1 --gas 300000000000000 `


# Paginated methods

## recover the reward token info by the address

> `near view $CONTRACT_ID get_stake_ft_contract '{"stake_ft_contract":"nativo_token.testnet"}' `


### View all the token staked
`near view $CONTRACT_ID get_all_nfts_for_token '{"from_index":"0","limit":50}' `

### View all the token staked by status
  1. Stake has been started
    1. Running
  2.  Stake has been paused by the owner
    2. Suspended
  3.  Stake has been finished and can be claimed
    1. Finished
  4.  If the owner withdraw before the block period ends.
    1. Canceled
  5.  if the auction its ended and has a bid 
    1. Claimed
  6.  The Stake doesnt exist
    1. NotFound

`near view $CONTRACT_ID get_nfts_for_token_by_status '{"status":"Finished","from_index":"0","limit":50}' `

### get tokens staked paginated by owner and status
`near view $CONTRACT_ID get_tokens_by_owner_status '{"account_id":"dokxo_test.testnet","status":"Running","from_index":"0","limit":50}' `
### get tokens staked paginated by owner 
`near view $CONTRACT_ID get_tokens_by_owner '{"account_id":"dokxo_test.testnet","from_index":"0","limit":50}' `

