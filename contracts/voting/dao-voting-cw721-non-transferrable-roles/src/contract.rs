use crate::hooks::{stake_hook_msgs, unstake_hook_msgs};
#[cfg(not(feature = "library"))]
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    register_staked_nft, register_unstaked_nfts, Config, CONFIG, DAO, HOOKS, MAX_CLAIMS,
    NFT_BALANCES, NFT_CLAIMS, STAKED_NFTS_PER_OWNER, TOTAL_STAKED_NFTS,
};
use crate::ContractError;
use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw721::Cw721ReceiveMsg;
use cw_storage_plus::Bound;
use cw_utils::Duration;
use dao_interface::Admin;

pub(crate) const CONTRACT_NAME: &str = "crates.io:dao-voting-cw721-staked";
pub(crate) const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO use cw-ownable?
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<Empty>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    DAO.save(deps.storage, &info.sender)?;

    let owner = msg
        .owner
        .as_ref()
        .map(|owner| match owner {
            Admin::Address { addr } => deps.api.addr_validate(addr),
            Admin::CoreModule {} => Ok(info.sender),
        })
        .transpose()?;

    let config = Config {
        owner: owner.clone(),
        nft_address: deps.api.addr_validate(&msg.nft_address)?,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default()
        .add_attribute("method", "instantiate")
        .add_attribute("nft_contract", msg.nft_address)
        .add_attribute(
            "owner",
            owner
                .map(|a| a.into_string())
                .unwrap_or_else(|| "None".to_string()),
        ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<Empty>, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { owner } => execute_update_config(info, deps, owner),
        ExecuteMsg::AddHook { addr } => execute_add_hook(deps, info, addr),
        ExecuteMsg::RemoveHook { addr } => execute_remove_hook(deps, info, addr),
    }
}

pub fn execute_stake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    wrapper: Cw721ReceiveMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.nft_address {
        return Err(ContractError::InvalidToken {
            received: info.sender,
            expected: config.nft_address,
        });
    }
    let staker = deps.api.addr_validate(&wrapper.sender)?;
    register_staked_nft(deps.storage, env.block.height, &staker, &wrapper.token_id)?;
    let hook_msgs = stake_hook_msgs(deps.storage, staker.clone(), wrapper.token_id.clone())?;
    Ok(Response::default()
        .add_submessages(hook_msgs)
        .add_attribute("action", "stake")
        .add_attribute("from", staker)
        .add_attribute("token_id", wrapper.token_id))
}


pub fn execute_update_config(
    info: MessageInfo,
    deps: DepsMut,
    new_owner: Option<String>,
    duration: Option<Duration>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if config.owner.map_or(true, |owner| owner != info.sender) {
        return Err(ContractError::NotOwner {});
    }

    let new_owner = new_owner
        .map(|new_owner| deps.api.addr_validate(&new_owner))
        .transpose()?;

    config.owner = new_owner;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default()
        .add_attribute("action", "update_config")
        .add_attribute(
            "owner",
            config
                .owner
                .map(|a| a.to_string())
                .unwrap_or_else(|| "none".to_string()),
        )
}

pub fn execute_add_hook(
    deps: DepsMut,
    info: MessageInfo,
    addr: String,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    if config.owner.map_or(true, |owner| owner != info.sender) {
        return Err(ContractError::NotOwner {});
    }

    let hook = deps.api.addr_validate(&addr)?;
    HOOKS.add_hook(deps.storage, hook)?;

    Ok(Response::default()
        .add_attribute("action", "add_hook")
        .add_attribute("hook", addr))
}

pub fn execute_remove_hook(
    deps: DepsMut,
    info: MessageInfo,
    addr: String,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    if config.owner.map_or(true, |owner| owner != info.sender) {
        return Err(ContractError::NotOwner {});
    }

    let hook = deps.api.addr_validate(&addr)?;
    HOOKS.remove_hook(deps.storage, hook)?;

    Ok(Response::default()
        .add_attribute("action", "remove_hook")
        .add_attribute("hook", addr))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => query_config(deps),
        QueryMsg::Dao {} => query_dao(deps),
        QueryMsg::Hooks {} => query_hooks(deps),
        QueryMsg::VotingPowerAtHeight { address, height } => {
            query_voting_power_at_height(deps, env, address, height)
        }
        QueryMsg::TotalPowerAtHeight { height } => query_total_power_at_height(deps, env, height),
        QueryMsg::Info {} => query_info(deps),
    }
}

pub fn query_voting_power_at_height(
    deps: Deps,
    env: Env,
    address: String,
    height: Option<u64>,
) -> StdResult<Binary> {
    // TODO query the contract?
    unimplemented!();
    // let address = deps.api.addr_validate(&address)?;
    // let height = height.unwrap_or(env.block.height);
    // let power = NFT_BALANCES
    //     .may_load_at_height(deps.storage, &address, height)?
    //     .unwrap_or_default();
    // to_binary(&dao_interface::voting::VotingPowerAtHeightResponse { power, height })
}

pub fn query_total_power_at_height(deps: Deps, env: Env, height: Option<u64>) -> StdResult<Binary> {
    // TODO query the contract?
    unimplemented!();
    // let height = height.unwrap_or(env.block.height);
    // let power = TOTAL_STAKED_NFTS
    //     .may_load_at_height(deps.storage, height)?
    //     .unwrap_or_default();
    // to_binary(&dao_interface::voting::TotalPowerAtHeightResponse { power, height })
}

pub fn query_config(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    to_binary(&config)
}

pub fn query_dao(deps: Deps) -> StdResult<Binary> {
    let dao = DAO.load(deps.storage)?;
    to_binary(&dao)
}

pub fn query_hooks(deps: Deps) -> StdResult<Binary> {
    to_binary(&HOOKS.query_hooks(deps)?)
}

pub fn query_info(deps: Deps) -> StdResult<Binary> {
    let info = cw2::get_contract_version(deps.storage)?;
    to_binary(&dao_interface::voting::InfoResponse { info })
}