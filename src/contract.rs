use crate::ContractError::Unauthorized;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256,
};
use ethabi::{Address, Contract, Function, Param, ParamType, StateMutability, Token, Uint};
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::error::ContractError;
use crate::msg::{Deposit, ExecuteMsg, GetJobIdResponse, InstantiateMsg, PalomaMsg, QueryMsg};
use crate::state::{State, STATE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        job_id: msg.job_id.clone(),
        owner: info.sender.clone(),
    };
    STATE.save(deps.storage, &state)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("job_id", msg.job_id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<PalomaMsg>, ContractError> {
    match msg {
        ExecuteMsg::PutSwap { deposits } => swap(deps, deposits),
        ExecuteMsg::SetPaloma {} => set_paloma(deps, info),
        ExecuteMsg::UpdateCompass { new_compass } => update_compass(deps, info, new_compass),
        ExecuteMsg::UpdateRefundWallet { new_refund_wallet } => {
            update_refund_wallet(deps, info, new_refund_wallet)
        }
        ExecuteMsg::UpdateFee { fee } => update_fee(deps, info, fee),
    }
}

fn swap(deps: DepsMut, deposits: Vec<Deposit>) -> Result<Response<PalomaMsg>, ContractError> {
    #[allow(deprecated)]
    let contract: Contract = Contract {
        constructor: None,
        functions: BTreeMap::from_iter(vec![(
            "multiple_swap".to_string(),
            vec![Function {
                name: "multiple_swap".to_string(),
                inputs: vec![
                    Param {
                        name: "deposit_id".to_string(),
                        kind: ParamType::Array(Box::new(ParamType::Uint(256))),
                        internal_type: None,
                    },
                    Param {
                        name: "remaining_counts".to_string(),
                        kind: ParamType::Array(Box::new(ParamType::Uint(256))),
                        internal_type: None,
                    },
                    Param {
                        name: "amount_out_min".to_string(),
                        kind: ParamType::Array(Box::new(ParamType::Uint(256))),
                        internal_type: None,
                    },
                ],
                outputs: Vec::new(),
                constant: None,
                state_mutability: StateMutability::NonPayable,
            }],
        )]),
        events: BTreeMap::new(),
        errors: BTreeMap::new(),
        receive: false,
        fallback: false,
    };

    let mut tokens_id: Vec<Token> = vec![];
    let mut tokens_remaining_counts: Vec<Token> = vec![];
    let mut tokens_min_amount: Vec<Token> = vec![];
    for deposit in deposits {
        let deposit_id = deposit.deposit_id;
        let remaining_count = deposit.remaining_count;
        let amount_out_min = deposit.amount_out_min;
        tokens_id.push(Token::Uint(Uint::from_big_endian(
            &deposit_id.to_be_bytes(),
        )));
        tokens_remaining_counts.push(Token::Uint(Uint::from_big_endian(
            &remaining_count.to_be_bytes(),
        )));
        tokens_min_amount.push(Token::Uint(Uint::from_big_endian(
            &amount_out_min.to_be_bytes(),
        )));
    }

    let tokens = vec![
        Token::Array(tokens_id),
        Token::Array(tokens_remaining_counts),
        Token::Array(tokens_min_amount),
    ];
    let state = STATE.load(deps.storage)?;
    Ok(Response::new()
        .add_message(CosmosMsg::Custom(PalomaMsg {
            job_id: state.job_id,
            payload: Binary(
                contract
                    .function("multiple_swap")
                    .unwrap()
                    .encode_input(tokens.as_slice())
                    .unwrap(),
            ),
        }))
        .add_attribute("action", "multiple_swap"))
}

fn set_paloma(deps: DepsMut, info: MessageInfo) -> Result<Response<PalomaMsg>, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.owner != info.sender {
        return Err(Unauthorized {});
    }
    #[allow(deprecated)]
    let contract: Contract = Contract {
        constructor: None,
        functions: BTreeMap::from_iter(vec![(
            "set_paloma".to_string(),
            vec![Function {
                name: "set_paloma".to_string(),
                inputs: vec![],
                outputs: Vec::new(),
                constant: None,
                state_mutability: StateMutability::NonPayable,
            }],
        )]),
        events: BTreeMap::new(),
        errors: BTreeMap::new(),
        receive: false,
        fallback: false,
    };
    Ok(Response::new()
        .add_message(CosmosMsg::Custom(PalomaMsg {
            job_id: state.job_id,
            payload: Binary(
                contract
                    .function("set_paloma")
                    .unwrap()
                    .encode_input(&[])
                    .unwrap(),
            ),
        }))
        .add_attribute("action", "set_paloma"))
}

fn update_compass(
    deps: DepsMut,
    info: MessageInfo,
    new_compass: String,
) -> Result<Response<PalomaMsg>, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.owner != info.sender {
        return Err(Unauthorized {});
    }
    let new_compass_address: Address = Address::from_str(new_compass.as_str()).unwrap();
    #[allow(deprecated)]
    let contract: Contract = Contract {
        constructor: None,
        functions: BTreeMap::from_iter(vec![(
            "update_compass".to_string(),
            vec![Function {
                name: "update_compass".to_string(),
                inputs: vec![Param {
                    name: "new_compass".to_string(),
                    kind: ParamType::Address,
                    internal_type: None,
                }],
                outputs: Vec::new(),
                constant: None,
                state_mutability: StateMutability::NonPayable,
            }],
        )]),
        events: BTreeMap::new(),
        errors: BTreeMap::new(),
        receive: false,
        fallback: false,
    };

    Ok(Response::new()
        .add_message(CosmosMsg::Custom(PalomaMsg {
            job_id: state.job_id,
            payload: Binary(
                contract
                    .function("update_compass")
                    .unwrap()
                    .encode_input(&[Token::Address(new_compass_address)])
                    .unwrap(),
            ),
        }))
        .add_attribute("action", "update_compass"))
}

fn update_refund_wallet(
    deps: DepsMut,
    info: MessageInfo,
    new_refund_wallet: String,
) -> Result<Response<PalomaMsg>, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.owner != info.sender {
        return Err(Unauthorized {});
    }
    let new_refund_wallet_address: Address = Address::from_str(new_refund_wallet.as_str()).unwrap();
    #[allow(deprecated)]
    let contract: Contract = Contract {
        constructor: None,
        functions: BTreeMap::from_iter(vec![(
            "update_refund_wallet".to_string(),
            vec![Function {
                name: "update_refund_wallet".to_string(),
                inputs: vec![Param {
                    name: "new_refund_wallet".to_string(),
                    kind: ParamType::Address,
                    internal_type: None,
                }],
                outputs: Vec::new(),
                constant: None,
                state_mutability: StateMutability::NonPayable,
            }],
        )]),
        events: BTreeMap::new(),
        errors: BTreeMap::new(),
        receive: false,
        fallback: false,
    };

    Ok(Response::new()
        .add_message(CosmosMsg::Custom(PalomaMsg {
            job_id: state.job_id,
            payload: Binary(
                contract
                    .function("update_refund_wallet")
                    .unwrap()
                    .encode_input(&[Token::Address(new_refund_wallet_address)])
                    .unwrap(),
            ),
        }))
        .add_attribute("action", "update_refund_wallet"))
}

fn update_fee(
    deps: DepsMut,
    info: MessageInfo,
    fee: Uint256,
) -> Result<Response<PalomaMsg>, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.owner != info.sender {
        return Err(Unauthorized {});
    }
    #[allow(deprecated)]
    let contract: Contract = Contract {
        constructor: None,
        functions: BTreeMap::from_iter(vec![(
            "update_fee".to_string(),
            vec![Function {
                name: "update_fee".to_string(),
                inputs: vec![Param {
                    name: "new_fee".to_string(),
                    kind: ParamType::Uint(256),
                    internal_type: None,
                }],
                outputs: Vec::new(),
                constant: None,
                state_mutability: StateMutability::NonPayable,
            }],
        )]),
        events: BTreeMap::new(),
        errors: BTreeMap::new(),
        receive: false,
        fallback: false,
    };

    Ok(Response::new()
        .add_message(CosmosMsg::Custom(PalomaMsg {
            job_id: state.job_id,
            payload: Binary(
                contract
                    .function("update_fee")
                    .unwrap()
                    .encode_input(&[Token::Uint(Uint::from_big_endian(&fee.to_be_bytes()))])
                    .unwrap(),
            ),
        }))
        .add_attribute("action", "update_fee"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetJobId {} => to_binary(&get_job_id(deps)?),
    }
}

pub fn get_job_id(deps: Deps) -> StdResult<GetJobIdResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetJobIdResponse {
        job_id: state.job_id,
    })
}
