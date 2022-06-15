// #[cfg(test)]
// mod tests {
//     //TODO:use crate::parse_complex;
//     //https://gitlab.com/gitlab-org/security-products/demos/coverage-fuzzing/rust-fuzzing-example
//     use crate::tests::mock_querier::mock_dependencies_with_querier;
//     use crate::utils::compute_tax;
//     use crate::utils::decimal_division;
//     use crate::utils::decimal_multiplication;
//     // use crate::utils::decimal_subtraction;
//     use crate::utils::deduct_tax;
//     use crate::utils::only_allow_canon_addr;
//     use crate::utils::only_allow_human_addr;
//     use crate::utils::reverse_decimal;
//     use crate::utils::round_half_to_even_128;
//     use cosmwasm_std::to_binary;
//     use cosmwasm_std::Addr;
//     use cosmwasm_std::Api;
//     use cosmwasm_std::Binary;
//     use cosmwasm_std::CanonicalAddr;
//     use cosmwasm_std::Coin;
//     use cosmwasm_std::ContractResult;
//     use cosmwasm_std::Decimal;
//     use cosmwasm_std::MessageInfo;
//     use cosmwasm_std::Querier;
//     use cosmwasm_std::QuerierResult;
//     use cosmwasm_std::QueryRequest;
//     use cosmwasm_std::SystemError;
//     use cosmwasm_std::SystemResult;
//     use cosmwasm_std::Uint128;
//     use cosmwasm_std::{from_binary, StdError};
//     use std::str::FromStr;
//     use terra_cosmwasm::TaxRateResponse;
//     use terra_cosmwasm::TerraQuery;
//     use terra_cosmwasm::TerraQueryWrapper;
//     use terra_cosmwasm::TerraRoute;

//     #[test]
//     fn only_allow_human_addr_err_unauthorized() {
//         //Given
//         let sender = Addr::unchecked("owner");
//         let hacker = Addr::unchecked("hacker");
//         let message = MessageInfo {
//             sender: sender.clone(),
//             funds: [].to_vec(),
//         };
//         //When
//         let result = only_allow_human_addr(&message, &hacker.to_string());
//         let expected = Err(StdError::generic_err("unauthorized"));
//         //Then
//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn only_allow_human_addr_ok() {
//         //Given
//         let sender = Addr::unchecked("owner");
//         let message = MessageInfo {
//             sender: sender.clone(),
//             funds: [].to_vec(),
//         };
//         //When
//         let result = only_allow_human_addr(&message, &sender.to_string());
//         let expected = ();
//         //Then
//         assert_eq!(expected, result.unwrap());
//     }
//     //---------------------
//     #[test]
//     fn only_allow_canon_addr_err_unauthorized() {
//         //Given
//         let sender = Addr::unchecked("owner");
//         let hacker = Addr::unchecked("hacker");
//         let contract_balance: &[Coin] = &[];
//         let message = MessageInfo {
//             sender: sender.clone(),
//             funds: [].to_vec(),
//         };
//         let deps = mock_dependencies_with_querier(contract_balance);
//         let contract_caller_addr = CanonicalAddr::from(hacker.as_bytes());
//         //When
//         let result = only_allow_canon_addr(&deps.api, &message, &contract_caller_addr);
//         let expected = Err(StdError::generic_err("unauthorized"));
//         //Then
//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn only_allow_canon_addr_ok() {
//         //Given
//         let owner = "owner";
//         let sender = Addr::unchecked(owner);
//         let contract_balance: &[Coin] = &[];
//         let message = MessageInfo {
//             sender: sender.clone(),
//             funds: [].to_vec(),
//         };
//         let deps = mock_dependencies_with_querier(contract_balance);
//         let contract_caller_addr = deps.api.addr_canonicalize(sender.as_str()).unwrap();
//         //When
//         let result = only_allow_canon_addr(deps.as_ref().api, &message, &contract_caller_addr);
//         let expected = ();
//         //Then
//         assert_eq!(expected, result.unwrap());
//     }

//     #[test]
//     fn decimal_round_half_to_even_works() {
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("0.4").unwrap()),
//             Uint128::new(0)
//         );
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("0.5").unwrap()),
//             Uint128::new(0)
//         );
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("0.5000001").unwrap()),
//             Uint128::new(1)
//         );
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("1").unwrap()),
//             Uint128::new(1)
//         );
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("1.5").unwrap()),
//             Uint128::new(2)
//         );
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("54.1754").unwrap()),
//             Uint128::new(54)
//         );
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("747.1754").unwrap()),
//             Uint128::new(747)
//         );
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("747.499999999999").unwrap()),
//             Uint128::new(747)
//         );
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("747.5").unwrap()),
//             Uint128::new(748)
//         );
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("747.500000000001").unwrap()),
//             Uint128::new(748)
//         );
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("88191.89898934588").unwrap()),
//             Uint128::new(88192)
//         );
//         assert_eq!(
//             round_half_to_even_128(Decimal::from_str("88192.89898934588").unwrap()),
//             Uint128::new(88193)
//         );
//     }

//     #[test]
//     fn decimal_division_divide_by_zero() {
//         let result = decimal_division(Decimal::from_str("123").unwrap(), Decimal::zero());
//         let expected = Err(StdError::generic_err("b is zero"));
//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn decimal_division_ok() {
//         let result = decimal_division(
//             Decimal::from_str("1").unwrap(),
//             Decimal::from_str("0.5").unwrap(),
//         );
//         let expected: Decimal = Decimal::from_str("2").unwrap();
//         assert_eq!(expected, result.unwrap());
//     }

//     #[test]
//     fn decimal_multiplication_ok() {
//         //Unity mul
//         let result = decimal_multiplication(Decimal::one(), Decimal::one());
//         let expected = Decimal::one();
//         assert_eq!(expected, result);

//         //mul Zero equals zero
//         let result = decimal_multiplication(Decimal::zero(), Decimal::one());
//         let expected = Decimal::zero();
//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn reverse_decimal_err_divide_by_zero() {
//         //Unity mul
//         let result = reverse_decimal(Decimal::zero());
//         let expected = Err(StdError::generic_err("decimal is zero"));
//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn reverse_decimal_ok() {
//         //inv one equals one
//         let result = reverse_decimal(Decimal::one());
//         let expected = Decimal::one();
//         assert_eq!(expected, result.unwrap());
//     }

//     //TODO: Add more test changing tax_rate and tax_cape
//     #[test]
//     fn compute_tax_ok() {
//         //Given
//         let owner = "owner";
//         let sender = Addr::unchecked(owner);
//         let contract_balance: &[Coin] = &[];
//         let deps = mock_dependencies_with_querier(contract_balance);
//         let contract_caller_addr = CanonicalAddr::from(sender.as_bytes());
//         assert_eq!(contract_caller_addr.as_slice(), sender.as_bytes());

//         let user_coin = Coin::new(1000000, "uust");

//         //When

//         let result = compute_tax(deps.as_ref(), &user_coin);
//         let expected: Uint128 = Uint128::new(9901);

//         //Then
//         assert_eq!(expected, result.unwrap());
//     }

//     #[test]
//     fn deduct_tax_ok() {
//         //Given
//         let owner = "owner";
//         let sender = Addr::unchecked(owner);
//         let contract_balance: &[Coin] = &[];
//         let deps = mock_dependencies_with_querier(contract_balance);
//         let contract_caller_addr = CanonicalAddr::from(sender.as_bytes());
//         assert_eq!(contract_caller_addr.as_slice(), sender.as_bytes());

//         let user_coin = Coin::new(1000000, "uust");

//         //When

//         let result = deduct_tax(deps.as_ref(), user_coin);
//         let expected: Coin = Coin::new(990099, "uust");

//         //Then
//         assert_eq!(expected, result.unwrap());
//     }

//     #[test]
//     fn querier_ok() {
//         //Given
//         //let sender = Addr::unchecked("owner");
//         let contract_balance: &[Coin] = &[Coin::new(990099, "uusd"), Coin::new(1000000, "uluna")];
//         let deps = mock_dependencies_with_querier(contract_balance);
//         let route: TerraRoute = TerraRoute::Treasury;
//         let query_data: TerraQuery = TerraQuery::TaxRate { /* fields */ };
//         let request = QueryRequest::Custom(TerraQueryWrapper { route, query_data });
//         let bin_request = to_binary(&request);

//         //When
//         let result: QuerierResult = deps.querier.raw_query(&bin_request.unwrap());
//         let contract_result_binary: ContractResult<Binary> = result.unwrap();
//         let final_response: TaxRateResponse =
//             from_binary(&contract_result_binary.unwrap()).unwrap();

//         let expected = TaxRateResponse {
//             rate: Decimal::from_str("0.01").unwrap(),
//         };
//         //Then
//         assert_eq!(expected, final_response);
//     }

//     #[test]
//     fn querier_custom_err_bad_route() {
//         //Given
//         //let sender = Addr::unchecked("owner");
//         let contract_balance: &[Coin] = &[Coin::new(990099, "uusd"), Coin::new(1000000, "uluna")];
//         let deps = mock_dependencies_with_querier(contract_balance);
//         let route: TerraRoute = TerraRoute::Market;
//         let query_data: TerraQuery = TerraQuery::TaxRate { /* fields */ };
//         let request = QueryRequest::Custom(TerraQueryWrapper { route, query_data });
//         let bin_request = to_binary(&request);

//         //When
//         let result: QuerierResult = deps.querier.raw_query(&bin_request.unwrap());
//         let expected = SystemResult::Err(SystemError::UnsupportedRequest {
//             kind: "route type Not Found".to_string(),
//         });
//         //Then
//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn querier_custom_err_bad_query_data() {
//         //Given
//         //let sender = Addr::unchecked("owner");
//         let contract_balance: &[Coin] = &[Coin::new(990099, "uusd"), Coin::new(1000000, "uluna")];
//         let deps = mock_dependencies_with_querier(contract_balance);
//         let route: TerraRoute = TerraRoute::Treasury;
//         let query_data: TerraQuery = TerraQuery::Swap {
//             offer_coin: Coin::new(1000000, "uluna"),
//             ask_denom: "uluna".to_string(),
//         };
//         let request = QueryRequest::Custom(TerraQueryWrapper { route, query_data });
//         let bin_request = to_binary(&request);

//         //When
//         let result: QuerierResult = deps.querier.raw_query(&bin_request.unwrap());
//         let expected = SystemResult::Err(SystemError::UnsupportedRequest {
//             kind: "query_data type Not Found".to_string(),
//         });
//         //Then
//         assert_eq!(expected, result);
//     }
// }
