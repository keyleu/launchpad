use crate::common_setup::setup_minter::common::constants::MINT_PRICE;
use crate::common_setup::setup_minter::vending_minter::mock_params::mock_create_minter;
use crate::common_setup::setup_minter::vending_minter::setup::build_init_msg;
use crate::common_setup::{
    setup_minter::common::constants::CREATION_FEE, templates::vending_minter_template,
};
use base_factory::ContractError;
use cosmwasm_std::{coin, coins, Empty, Timestamp};
use cw_multi_test::{BankSudo, Executor, SudoMsg};
use sg2::msg::Sg2ExecuteMsg;
use sg2::query::{ParamsResponse, Sg2QueryMsg};
use sg2::tests::mock_collection_params;
use sg_std::{GENESIS_MINT_START_TIME, NATIVE_DENOM};
use vending_factory::msg::{
    SudoMsg as VendingFactorySudoMsg, VendingUpdateParamsExtension, VendingUpdateParamsMsg,
};

#[test]
fn frozen_factory_cannot_create_new_minters() {
    let vt = vending_minter_template(2);
    let (mut router, creator, _) = (vt.router, vt.accts.creator, vt.accts.buyer);
    let factory = vt.collection_response_vec[0].factory.clone().unwrap();
    let num_tokens = 2;

    // factory is not frozen
    let res: ParamsResponse<Empty> = router
        .wrap()
        .query_wasm_smart(factory.clone(), &Sg2QueryMsg::Params {})
        .unwrap();
    assert!(!res.params.frozen);

    // update factory to frozen
    let extension = VendingUpdateParamsExtension {
        max_token_limit: None,
        max_per_address_limit: None,
        airdrop_mint_price: None,
        airdrop_mint_fee_bps: None,
        shuffle_fee: None,
    };
    let update_msg = VendingUpdateParamsMsg {
        add_sg721_code_ids: None,
        rm_sg721_code_ids: None,
        frozen: Some(true),
        code_id: None,
        creation_fee: None,
        min_mint_price: None,
        mint_fee_bps: None,
        max_trading_offset_secs: None,
        extension,
    };
    let sudo_msg = VendingFactorySudoMsg::UpdateParams(Box::new(update_msg));
    let res = router.wasm_sudo(factory.clone(), &sudo_msg);
    assert!(res.is_ok());

    // add funds to creator
    router
        .sudo(SudoMsg::Bank({
            BankSudo::Mint {
                to_address: creator.to_string(),
                amount: coins(CREATION_FEE - 2_000_000_000, NATIVE_DENOM),
            }
        }))
        .map_err(|err| println!("{:?}", err))
        .ok();

    // creating new minter throws error
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let mut msg = mock_create_minter(None, mock_collection_params(), Some(start_time));
    msg.init_msg = build_init_msg(
        None,
        msg.clone(),
        num_tokens,
        coin(MINT_PRICE, NATIVE_DENOM),
    );
    msg.collection_params.info.creator = creator.to_string();
    let creation_fee = coins(CREATION_FEE, NATIVE_DENOM);
    let msg = Sg2ExecuteMsg::CreateMinter(msg);
    let res = router.execute_contract(creator, factory, &msg, &creation_fee);
    assert_eq!(
        res.unwrap_err().source().unwrap().to_string(),
        ContractError::Frozen {}.to_string()
    );
}

#[test]
fn create_minter_with_ibc_asset_mint_price() {
    let vt = vending_minter_template(2);
    let (mut router, creator, _) = (vt.router, vt.accts.creator, vt.accts.buyer);
    let factory = vt.collection_response_vec[0].factory.clone().unwrap();
    let num_tokens = 2;

    // add funds to creator
    router
        .sudo(SudoMsg::Bank({
            BankSudo::Mint {
                to_address: creator.to_string(),
                amount: coins(CREATION_FEE - 2_000_000_000, NATIVE_DENOM),
            }
        }))
        .map_err(|err| println!("{:?}", err))
        .ok();

    // creating new minter throws error
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let mut msg = mock_create_minter(None, mock_collection_params(), Some(start_time));
    let ibc_denom = "ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858";
    msg.init_msg = build_init_msg(None, msg.clone(), num_tokens, coin(1000u128, ibc_denom));
    msg.collection_params.info.creator = creator.to_string();
    let creation_fee = coins(CREATION_FEE, NATIVE_DENOM);
    let msg = Sg2ExecuteMsg::CreateMinter(msg);
    let res = router.execute_contract(creator, factory, &msg, &creation_fee);
    println!("{:?}", res);
    assert!(res.is_ok());
    // assert_eq!(
    //     res.unwrap_err().source().unwrap().to_string(),
    //     ContractError::Frozen {}.to_string()
    // );
}
