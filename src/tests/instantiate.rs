use cosmwasm_std::{Env, MessageInfo, Response, Uint128};
use cosmwasm_std::testing::{mock_env, mock_info};
use cw20::Denom;

use valkyrie_qualifier::QualifiedContinueOption;

use crate::executions::{ExecuteResult, instantiate};
use crate::msgs::InstantiateMsg;
use crate::states::{QualifierConfig, Requirement};
use crate::tests::*;
use crate::tests::mock_querier::{custom_deps, CustomDeps};

pub fn exec(
    deps: &mut CustomDeps,
    env: Env,
    info: MessageInfo,
    min_token_balances: Vec<(Denom, Uint128)>,
    min_luna_staking: Uint128,
    participation_limit: u64,
    min_send_amount: Uint128,
) -> ExecuteResult {
    let msg = InstantiateMsg {
        min_token_balances,
        min_luna_staking,
        participation_limit,
        min_burn_amount: min_send_amount,
    };

    instantiate(deps.as_mut(), env, info, msg)
}

pub fn will_success(
    deps: &mut CustomDeps,
    min_token_balances: Vec<(Denom, Uint128)>,
    min_luna_staking: Uint128,
    participation_limit: u64,
    min_send_amount: Uint128,
) -> (Env, MessageInfo, Response) {
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);

    let response = exec(
        deps,
        env.clone(),
        info.clone(),
        min_token_balances,
        min_luna_staking,
        participation_limit,
        min_send_amount,
    ).unwrap();

    (env, info, response)
}

pub fn default(deps: &mut CustomDeps) -> (Env, MessageInfo, Response) {
    will_success(
        deps,
        vec![(Denom::Native(MIN_TOKEN_BALANCE_DENOM_NATIVE.to_string()), MIN_TOKEN_BALANCE_AMOUNT)],
        MIN_LUNA_STAKING,
        PARTICIPATION_LIMIT,
        MIN_BURN_AMOUNT,
    )
}

#[test]
fn succeed() {
    let mut deps = custom_deps();

    let (_, info, _) = default(&mut deps);

    let config = QualifierConfig::load(&deps.storage).unwrap();
    assert_eq!(config, QualifierConfig {
        admin: info.sender,
        continue_option_on_fail: QualifiedContinueOption::Ineligible,
    });

    let requirement = Requirement::load(&deps.storage).unwrap();
    assert_eq!(requirement, Requirement {
        min_token_balances: vec![(Denom::Native(MIN_TOKEN_BALANCE_DENOM_NATIVE.to_string()), MIN_TOKEN_BALANCE_AMOUNT)],
        min_luna_staking: MIN_LUNA_STAKING,
        participation_limit: PARTICIPATION_LIMIT,
        min_burn_amount: MIN_BURN_AMOUNT,
    });
}
