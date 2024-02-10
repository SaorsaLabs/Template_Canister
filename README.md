# example_canister

This repo provides an example canister (Smart Contract) which can be used on the [Internet Computer Protocol](https://internetcomputer.org/). The canister is designed to allow developers to build on top of a core set of features while optionally including bolt-on features such as HTTP Outcalls, Timers and Stable Memory Storage. 
The minimum code required to compile the canister is the core module. On top of this you can add your own modules or use any/ all of the example modules.

## Getting Started
If you have not deployed a smart contact on the Internet Computer before, check out this guide to building on the IC using rust [here]( https://internetcomputer.org/docs/current/developer-docs/backend/rust/)

1. Clone this repository and place it in your folder of choice. 
2. You can choose to either deploy the canister on a local testnet or on the ICP mainnet. Note the inter-canister-call method and HTTP outcall method will only work on mainnet. 

#### To deploy locally
first start a local dfx environment 

```bash
dfx start –background --clean
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

3. Once deployed on mainnet there are a couple of way to interact and test the canister. You can continue to make calls through your DFX terminal or you can interact with it through the official [Internet Computer Blockchain Dashboard]( https://dashboard.internetcomputer.org/)

To use the blockchain explorer UI – first get your canister ID from the canister_ids.json file and then search for this on the IC Dashboard. This will show you the publicly visible methods defined in the project’s .did file. Calling most of these methods will result in an ‘unauthorized’ error because the canister implements admin/ authorised user ‘gates’. 

If you want to call all methods from the dashboard you can add the ‘anonymous principal’ as an admin and authorised user. Note – this will essentially remove all ‘gates’ allowing anyone to call all the canister methods. 

```bash
dfx canister call example_canister --network ic add_admin '("2vxsx-fae")'
dfx canister call example_canister --network ic add_authorised '("2vxsx-fae")'
``` 

4. Test Arguments 
The core methods in the example_canister should be self explanatory. We have added a number of other methods to demo some of the other functionality added with the HTTP, timer and custom modules.  

#### Write to Stable Storage
The internet computer allows canister to store data in volatile or stable memory. For ease, most of the data associated in the core functions is stored in volatile memory. It should be noted that because the canister is replicated multiple times across a subnet, volatile memory is not at risk of being lost if a node goes ‘down’. There is however only a limited amount of volatile memory a canister can use (currently 4gb). 
To store more data, canisters can use stable memory (currently up to 96gb). There are two rust crates that assist in writing data to stable memory (ic-stable-structures and ic-stable-memory). The example_canister uses [ic-stable-structures]( https://docs.rs/ic-stable-structures/latest/ic_stable_structures/) which was created by Dfinity. 

In this canister, the example_custom_module (btree_logic.rs) defines the logic for interacting with a Btreemap stored in stable memory. The BTreeMap is defined in stable_memory.rs in the core module. 

Add entry to the Map. Input is a String (Name), String (Nickname), u64 (age). 
```bash
dfx canister call example_canister --network ic add_btree_method '("Jonathan", "Jono", 28: nat64)'
```

Lookup entry in the Map
```bash
dfx canister call example_canister --network ic get_value_btree_method '("Jonathan")'
```

Remove entry from the Map
```bash
dfx canister call example_canister --network ic remove_btree_method '("Jonathan")'
```
