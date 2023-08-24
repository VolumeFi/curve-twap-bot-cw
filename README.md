# Curve TWAP-Bot CosmWasm smart contract on Paloma

This is a CosmWasm smart contract to send messages to a TWAP-bot smart contract on EVM chain written in Vyper.

Users can deposit their token or coin into a Vyper smart contract on EVM chain.

There is a view function in the smart contract that returns swappable id, minimal out amount, and remaining trade counts that can be swapped on Curve.

A scheduler or script fetch the swappable id from the Vyper smart contract and run `multiple_swap` function with the id and minimal out amount.

And then, the Vyper smart contract will swap the assets and sent them to the depositors.

## ExecuteMsg

### PutSwap

Run `multiple_swap` function on Vyper smart contract.

| Key            | Type         | Description                        |
|----------------|--------------|------------------------------------|
| deposits       | Vec<Deposit> | Deposit information vector to Swap |

### SetPaloma

Run `set_paloma` function on Vyper smart contract to set CW address in `bytes32`.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |

### UpdateCompass

Run `update_compass` function on Vyper smart contract to update compass-evm contract address.

| Key         | Type   | Description                      |
|-------------|--------|----------------------------------|
| new_compass | String | New compass-evm contract address |

### UpdateRefundWallet

Run `update_refund_wallet` function on Vyper smart contract to update refund wallet address that receives gas fee.

| Key               | Type   | Description               |
|-------------------|--------|---------------------------|
| new_refund_wallet | String | New refund wallet address |

### UpdateFee

Run `update_fee` function on Vyper smart contract to update gas fee to pay for users.

| Key | Type    | Description    |
|-----|---------|----------------|
| fee | Uint256 | New fee amount |


## Struct

### Deposit

| Key             | Type    | Description                                  |
|-----------------|---------|----------------------------------------------|
| deposit_id      | u32     | Deposit id to swap on a Vyper smart contract |
| remaining_count | u32     | Current remaining count of the TWAP bot      |
| out_amount_min  | Uint256 | Minimal amount to prevent front running      |

## QueryMsg

### GetJobId

Get `job_id` of Paloma message to run `multiple_withdraw` function on a Vyper smart contract.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |

#### Response

| Key    | Type   | Description      |
|--------|--------|------------------|
| job_id | String | Job Id on Paloma |
