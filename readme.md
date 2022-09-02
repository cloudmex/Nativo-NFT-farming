# Nativo NFT - *Staking*

# ![Logo](https://develop.testnet.nativonft.app/static/media/LogoBlanco.30fcfa22.png)
<br>
NFT staking allows you to farm $Ntv as reward blocking your NFT by tim period without loosing the ownership of your NFT's

1. Secure your NFT in NFT staking and request a reward of $NTV tokens
2. Suscribe your NFT to events collections and get more rewards($NTV tokens + owner collection rewards).

## Prepare your env :

### Dev account

> `export CONTRACT_ID="dev-1662064742631-56583329092440"`

### Sub account

> `export CONTRACT_ID="staking.nativo_mkt.testnet"  `

### Make a devdeploy run:

> `./devdeploy.sh`

### Make a deploy run:

> `./build.sh`

### Make a migration run:

> `./migrate.sh`

## Initializing the contract

> `near call $CONTRACT_ID new '{"owner_account_id": "dokxo.testnet","treasury_account_id": "dokxo.testnet","contract_interest": 800,"contract_fee": 200 }' --accountId dokxo.testnet`

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


## recover the reward token by th stake id

### 'nativo_token.testnet' is the id 1

> `near view $CONTRACT_ID get_stake_ft_contract '{"stake_ft_contract":"nativo_token.testnet"}' `

### Recover a token stake info

> `near view $CONTRACT_ID get_staking_token_by_id '{"st_token":0}'  `


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

### calculate the reward for a staked token
> `near call $CONTRACT_ID calculate_reward_nft '{"st_token":2}' --accountId dokxo_test.testnet  `

### withdraw a nft from stake
### you must accept ot decline the withdraw and attach a penalty fee, if the time isnt over yet we'll take the penalty and 
### send just the nft without the rewards earned.
> `near call $CONTRACT_ID withdraw_nft_owner '{"st_token":0,"accept_withdraw":true}' --accountId dokxo.testnet --deposit 0.2`


# Stake a NFT

### Ask for a auctioning - Mintbase

> `near call alst77.mintspace2.testnet nft_transfer_call '{"receiver_id": "dev-1648670267690-23487881027419","token_id":"0", "msg": "{\"description\": \"list a new nft for auctioning\", \"auction_amount_requested\": 100000000000000000000000000 }"}' --accountId alan_test.testnet --depositYocto 1 --gas 300000000000000`

### Ask for a auctioning - Paras Id

> `near call paras-token-v2.testnet nft_transfer_call '{"receiver_id": "dev-1647921766612-74437195022952","token_id": "299:9", "msg": "{\"description\": \"list my nft for auctioning\", \"auction_requested\": \"100000000000000000000000000\"}"}' --accountId alan_test.testnet --depositYocto 1 --gas 300000000000000`

### Ask for a auctioning - Nativo NFT

> `near call minterv2.nativo-minter.testnet nft_transfer_call '{"receiver_id": "dev-1661277753358-64242303541632","token_id":"48", "msg": "{\"_type\": \"token\",\"reward_token\": \"nativo_token.testnet\" ,\"blocked_period\": 1 }"}' --accountId dokxo_test.testnet --depositYocto 1 --gas 300000000000000 `

### Recover a token from a dev account

> `near call minterv2.nativo-minter.testnet nft_transfer '{"receiver_id": "dokxo_test.testnet", "token_id": "48", "memo": "Go Team :)"}' --accountId dev-1661277753358-64242303541632 --depositYocto 1  `



### create a event stake

 > `near call $CONTRACT_ID create_event_for_nfts '{"event_info": {"event_owner": "dokxo.testnet","event_tittle": "title","event_description": "descrip","event_media": "media","nft_contract": "minterv2.nativo-minter.testnet","event_time": 1234353454,"event_start_at": 1235677888,"event_blocked_until": 123456789,"reward_token": ["nativo-token.testnet"],"reward_accumulated": []}}' --accountId dokxo_test.testnet  `
 