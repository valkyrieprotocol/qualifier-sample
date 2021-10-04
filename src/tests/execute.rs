use cosmwasm_std::{coin, Env, MessageInfo, Response, Uint128, SubMsg, CosmosMsg, BankMsg};
use cosmwasm_std::testing::{mock_env, mock_info};

use crate::executions::{ExecuteResult, execute};
use crate::tests::mock_querier::{custom_deps, CustomDeps};
use crate::states::EXECUTIONS;

pub fn exec(
    deps: &mut CustomDeps,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> ExecuteResult {
    execute(deps.as_mut(), env, info, amount)
}

pub fn will_success(deps: &mut CustomDeps, actor: String, amount: Uint128) -> (Env, MessageInfo, Response) {
    let env = mock_env();
    let info = mock_info(actor.as_str(), &[]);

    let response = exec(deps, env.clone(), info.clone(), amount).unwrap();

    (env, info, response)
}

#[test]
fn succeed() {
    let mut deps = custom_deps();

    super::instantiate::default(&mut deps);

    let (_, info, response) = will_success(
        &mut deps,
        "Actor".to_string(),
        Uint128::new(1000),
    );
    assert_eq!(response.messages, vec![
        SubMsg::new(CosmosMsg::Bank(BankMsg::Burn {
            amount: vec![coin(1000, "uluna")],
        })),
    ]);

    let execution = EXECUTIONS.load(&deps.storage, &info.sender).unwrap();
    assert_eq!(execution, 1);
}

#[test]
fn succeed_dissatisfy_execution() {
    let mut deps = custom_deps();

    super::instantiate::default(&mut deps);

    let (_, info, response) = will_success(
        &mut deps,
        "Actor".to_string(),
        Uint128::new(10),
    );
    assert_eq!(response.messages, vec![
        SubMsg::new(CosmosMsg::Bank(BankMsg::Burn {
            amount: vec![coin(10, "uluna")],
        })),
    ]);

    let execution = EXECUTIONS.may_load(&deps.storage, &info.sender).unwrap();
    assert_eq!(execution, None);
}
