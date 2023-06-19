use cosmwasm_std::Timestamp;
use cw_multi_test::Executor;
use sg_std::GENESIS_MINT_START_TIME;

use open_edition_minter::msg::{EndTimeResponse, StartTimeResponse};
use open_edition_minter::msg::{ExecuteMsg, QueryMsg};

use crate::common_setup::templates::open_edition_minter_custom_template;

#[test]
fn check_start_end_time_updates() {
    let vt =
        open_edition_minter_custom_template(None, None, None, Some(10), Some(2), None, None, None)
            .unwrap();
    let (mut router, creator, _buyer) = (vt.router, vt.accts.creator, vt.accts.buyer);
    let minter_addr = vt.collection_response_vec[0].minter.clone().unwrap();

    // Query Start Time
    // We know it is GENESIS_MINT_START_TIME + 100
    let query_start_time_msg: QueryMsg = QueryMsg::StartTime {};
    let res: StartTimeResponse = router
        .wrap()
        .query_wasm_smart(minter_addr.clone(), &query_start_time_msg)
        .unwrap();
    assert_eq!(
        res.start_time,
        Timestamp::from_nanos(GENESIS_MINT_START_TIME + 100).to_string()
    );

    // Query End Time
    // We know it is GENESIS_MINT_START_TIME + 10_000
    let query_end_time_msg: QueryMsg = QueryMsg::EndTime {};
    let res: EndTimeResponse = router
        .wrap()
        .query_wasm_smart(minter_addr.clone(), &query_end_time_msg)
        .unwrap();
    assert_eq!(
        res.end_time,
        Timestamp::from_nanos(GENESIS_MINT_START_TIME + 10_000).to_string()
    );

    // Cant change start time to before the current time
    let new_start_time_msg =
        ExecuteMsg::UpdateStartTime(Timestamp::from_nanos(1_500_000_000_000_000_000));
    let res = router.execute_contract(
        creator.clone(),
        minter_addr.clone(),
        &new_start_time_msg,
        &[],
    );
    assert_eq!(
        res.err().unwrap().source().unwrap().to_string(),
        "InvalidStartTime 1500000000.000000000 < 1571797419.879305533"
    );

    // Cant change start time to after the end time
    let new_start_time_msg =
        ExecuteMsg::UpdateStartTime(Timestamp::from_nanos(GENESIS_MINT_START_TIME + 10_001));
    let res = router.execute_contract(
        creator.clone(),
        minter_addr.clone(),
        &new_start_time_msg,
        &[],
    );
    assert_eq!(
        res.err().unwrap().source().unwrap().to_string(),
        "InvalidStartTime 1647032400.000010000 < 1647032400.000010001"
    );

    // Cant change end time to before the current time
    let new_end_time_msg =
        ExecuteMsg::UpdateEndTime(Timestamp::from_nanos(1_500_000_000_000_000_000));
    let res = router.execute_contract(creator.clone(), minter_addr.clone(), &new_end_time_msg, &[]);
    assert_eq!(
        res.err().unwrap().source().unwrap().to_string(),
        "InvalidEndTime 1500000000.000000000 < 1571797419.879305533"
    );

    // Cant change end time to before the start time
    let new_end_time_msg =
        ExecuteMsg::UpdateEndTime(Timestamp::from_nanos(GENESIS_MINT_START_TIME + 99));
    let res = router.execute_contract(creator.clone(), minter_addr.clone(), &new_end_time_msg, &[]);
    assert_eq!(
        res.err().unwrap().source().unwrap().to_string(),
        "InvalidEndTime 1647032400.000000099 < 1647032400.000000100"
    );

    // Make valid change to start time
    let new_start_time_msg =
        ExecuteMsg::UpdateStartTime(Timestamp::from_nanos(GENESIS_MINT_START_TIME + 1_000));
    let res = router.execute_contract(
        creator.clone(),
        minter_addr.clone(),
        &new_start_time_msg,
        &[],
    );
    assert!(res.is_ok());

    // Query to validate the new start time
    let query_start_time_msg: QueryMsg = QueryMsg::StartTime {};
    let res: StartTimeResponse = router
        .wrap()
        .query_wasm_smart(minter_addr.clone(), &query_start_time_msg)
        .unwrap();
    assert_eq!(
        res.start_time,
        Timestamp::from_nanos(GENESIS_MINT_START_TIME + 1_000).to_string()
    );

    // Make valid change to end time
    let new_end_time_msg =
        ExecuteMsg::UpdateEndTime(Timestamp::from_nanos(GENESIS_MINT_START_TIME + 20_000));
    let res = router.execute_contract(creator, minter_addr.clone(), &new_end_time_msg, &[]);
    assert!(res.is_ok());

    // Query to validate the new end time
    let query_end_time_msg: QueryMsg = QueryMsg::EndTime {};
    let res: EndTimeResponse = router
        .wrap()
        .query_wasm_smart(minter_addr, &query_end_time_msg)
        .unwrap();
    assert_eq!(
        res.end_time,
        Timestamp::from_nanos(GENESIS_MINT_START_TIME + 20_000).to_string()
    );
}
