use std::ops::Deref;
use cosmwasm_std::{Deps, Order, QuerierWrapper, StdResult};
use token_bindings::TokenFactoryQuery;
use crate::abc::CurveFn;
use crate::msg::{CommonsPhaseConfigResponse, CurveInfoResponse, DonationsResponse};
use crate::state::{CURVE_STATE, CurveState, DONATIONS, PHASE, PHASE_CONFIG};

/// Get the current state of the curve
pub fn query_curve_info(
    deps: Deps<TokenFactoryQuery>,
    curve_fn: CurveFn,
) -> StdResult<CurveInfoResponse> {
    let CurveState {
        reserve,
        supply,
        reserve_denom,
        decimals,
        funding,
    } = CURVE_STATE.load(deps.storage)?;

    // This we can get from the local digits stored in instantiate
    let curve = curve_fn(decimals);
    let spot_price = curve.spot_price(supply);

    Ok(CurveInfoResponse {
        reserve,
        supply,
        funding,
        spot_price,
        reserve_denom,
    })
}

/// Load and return the phase config
pub fn query_phase_config(deps: Deps<TokenFactoryQuery>) -> StdResult<CommonsPhaseConfigResponse> {
    let phase = PHASE.load(deps.storage)?;
    let phase_config = PHASE_CONFIG.load(deps.storage)?;
    Ok(CommonsPhaseConfigResponse {
        phase_config,
        phase,
    })
}


// // TODO, maybe we don't need this
// pub fn get_denom(
//     deps: Deps<TokenFactoryQuery>,
//     creator_addr: String,
//     subdenom: String,
// ) -> GetDenomResponse {
//     let querier = TokenQuerier::new(&deps.querier);
//     let response = querier.full_denom(creator_addr, subdenom).unwrap();

//     GetDenomResponse {
//         denom: response.denom,
//     }
// }

pub fn query_donations(deps: Deps<TokenFactoryQuery>, start_aftor: Option<String>, limit: Option<u32>) -> StdResult<DonationsResponse> {
    let donations = cw_paginate::paginate_map(
        Deps {
            storage: deps.storage,
            api: deps.api,
            querier: QuerierWrapper::new(deps.querier.deref())
        },
        &DONATIONS,
        start_aftor.map(|addr| deps.api.addr_validate(&addr)).transpose()?.as_ref(),
        limit,
        Order::Descending,
    )?;

    Ok(DonationsResponse {
        donations,
    })
}