# Example Canister

This repo provides an example canister (Smart Contract) which can be used on the [Internet Computer Protocol](https://internetcomputer.org/). The canister is designed to allow developers to build on top of a core set of features while optionally including bolt-on features such as HTTP Outcalls, Timers and Stable Memory Storage. 
The minimum code required to compile the canister is the core module. On top of this you can add your own modules or use any/ all of the example modules.

## Getting Started
If you have not deployed a smart contact on the Internet Computer before, check out this guide to building on the IC using rust [here]( https://internetcomputer.org/docs/current/developer-docs/backend/rust/)

1. Clone this repository and place it in your folder of choice. 
2. You can choose to either deploy the canister on a local testnet or on the ICP mainnet. Note the inter-canister-call method and HTTP outcall method will only work on mainnet. 

#### To deploy locally
first start a local dfx environment 

```bash
dfx start --background --clean
``` 

This will start a clean version of dfx running in the background. In order to deploy the example canister you will need to know the principal of your developer identity. You can get this by calling `dfx identity get-principal` Simply add this to the argument in the deploy script below

```bash
dfx deploy example_canister --argument '(xxx-xxx-xxxx-xxxx)'
```

#### To deploy to mainnet
In order to deploy to mainnet you will need ‘cycles’ in your wallet. You can find more information about this [here]( https://internetcomputer.org/docs/current/developer-docs/getting-started/deploy/mainnet) . For main-net you can use the code below – remember to change the argument to your own developer principal. 

```bash
dfx deploy example_canister --network ic --argument '(xxxx-xxxx-xxx-xxxx-xxxx)'
```

## Making calls to the canister
Once deployed on mainnet there are a couple of way to interact and test the canister. You can continue to make calls through your DFX terminal or you can interact with it through the official [Internet Computer Blockchain Dashboard]( https://dashboard.internetcomputer.org/)

To use the blockchain explorer UI – first get your canister ID from the canister_ids.json file and then search for this on the IC Dashboard. This will show you the publicly visible methods defined in the project’s .did file. Calling most of these methods will result in an ‘unauthorized’ error because the canister implements admin/ authorised user ‘gates’. 

If you want to call all methods from the dashboard you can add the ‘anonymous principal’ as an admin and authorised user. NOTE - this does not completely remove the gate and other callers might still be blocked (for example other canister smart contracts who don't use the 'anonymous principal'). To have a method open to everyone, simply remove the gate code completely (see get_cycles_balance method in core/api). 

Example of adding the 'anonymous principal' as admin and authorised:

```bash
dfx canister call example_canister --network ic add_admin '("2vxsx-fae")'
dfx canister call example_canister --network ic add_authorised '("2vxsx-fae")'
``` 

## Test methods
The core methods in the example_canister should be self explanatory. We have added a number of other methods to demo some of the other functionality added with the HTTP, timer and custom modules.  

#### Write to Stable Storage
The internet computer allows canisters to store data in volatile or stable memory. For ease, most of the data associated in the core functions is stored in volatile memory. It should be noted that because the canister is replicated multiple times across a subnet, volatile memory is not at risk of being lost if a node goes ‘down’. There is however only a limited amount of volatile memory a canister can use (currently 4gb). 

To store more data, canisters can use stable memory (currently up to 400gb). There are two rust crates that assist in writing data to stable memory (ic-stable-structures and ic-stable-memory). The example_canister uses [ic-stable-structures]( https://docs.rs/ic-stable-structures/latest/ic_stable_structures/) which was created by Dfinity. 

In this canister, the example_custom_module (btree_logic.rs) defines the logic for interacting with a Btreemap stored in stable memory. The BTreeMap is defined in stable_memory.rs in the core module. 

Add entry to the Map. Input is a String (Name), String (Nickname), u64 (age). 
```bash
dfx canister call example_canister --network ic add_to_btree_map '("Jonathan", "J-dawg", 28: nat64)'
```

Lookup entry in the Map
```bash
dfx canister call example_canister --network ic get_value_btree_map '("Jonathan")'
```

Remove entry from the Map
```bash
dfx canister call example_canister --network ic remove_from_btree_map '("Jonathan")'
```

#### Call another canister 
Within the core module is a function to allow users to easily call other canister smart contracts on the Internet Computer (see utils.rs). An example of this being used is in example_custom_modlue/logic.rs which calls the ckBTC minter to fetch the estimated withdrawal fee.

```bash 
dfx canister call example_canister --network ic make_call_to_ckbtc_minter  
``` 

#### Make a HTTP outcall

Part of what makes the Internet Computer unlike any other blockchain is it’s ability to make HTTP calls to ‘off-chain’ endpoints and also serve HTTP web content directly from the canister smart contract.  

The HTTP module in the example canister provides basic examples of how to perform both inbound and outbound HTTP calls. 
Outbound call to Coinbase (fetching latest ICP price) 

```bash
dfx canister call example_canister --network ic  test_http_outcall
```

To test an inbound HTTP request to the text canister you can replace the xxxx’s with your deployed canister id. The route /data will redirect you to 221Bravo.App, /ok will display a Hello World message and /err will display another message

```html
https://xxxxx-xxxxx-xxxxxx-xxxxx-xxxx.icp0.io/data
https://xxxxx-xxxxx-xxxxxx-xxxxx-xxxx.icp0.io/ok
https://xxxxx-xxxxx-xxxxxx-xxxxx-xxxx.icp0.io/err
```

#### Start and Stop Timers

Timers are an incredibly useful feature of the Internet Computer. Timers can be used to call functions within your smart contract at a set interval. This can be really useful for making periodic calls to other smart contracts or even ‘self-calling’ a method on the canister using the timer. 
Start a timer (this text example logs a message to the canister logs every 30 seconds)

```bash
dfx canister call example_canister --network ic start_test_timer '(30: nat64)'
```

Stop all timers
```bash
dfx canister call example_canister --network ic stop_all_timers
```

#### Note
Timers are not persisted through upgrades. It is recommended to stop any timers before an upgrade and then restart them once the upgrade has been completed.