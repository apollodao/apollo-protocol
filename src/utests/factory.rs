use crate::factory::{next_dex_id, APOLLO_DEX_COUNT};
use cosmwasm_std::testing::mock_dependencies;
use test_case::test_case;

#[test_case(0u8 => 1u8; "init with 0, should return 1")]
#[test_case(255u8 => panics "attempt to add with overflow" ; "init with 255, should overflow")]
fn test_next_dex_id_ok(init: u8) -> u8 {
    // Given
    let mut deps = mock_dependencies();
    APOLLO_DEX_COUNT.save(&mut deps.storage, &init).unwrap();

    // When call the next_dex_id function with init records
    next_dex_id(&mut deps.storage).unwrap()
}
