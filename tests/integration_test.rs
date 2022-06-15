mod integration {
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    #[derive(Clone, PartialEq, Debug)]
    pub struct ApolloContracts {
        pub apollo_token: (Addr, u64),
    }

    pub fn mock_app() -> App {
        App::default()
    }

    fn contract_cw20() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw20_base::contract::execute,
            cw20_base::contract::instantiate,
            cw20_base::contract::query,
        );
        Box::new(contract)
    }

    fn init_contracts(app: &mut App, owner: &str) -> ApolloContracts {
        let cw20_code_id = app.store_code(contract_cw20());

        let apollo_token_instance = app
            .instantiate_contract(
                cw20_code_id,
                Addr::unchecked(owner.clone().to_string()),
                &cw20_base::msg::InstantiateMsg {
                    name: "APOLLO".to_string(),
                    symbol: "APOLLO".to_string(),
                    decimals: 6,
                    initial_balances: vec![],
                    mint: None,
                    marketing: None,
                },
                &[],
                String::from("APOLLO"),
                None,
            )
            .unwrap();
        return ApolloContracts {
            apollo_token: (apollo_token_instance, cw20_code_id),
        };
    }

    #[test]
    fn test_init_apollo_token() {
        // Given
        const OWNER: &str = "owner";
        let mut app: App = mock_app();
        let expected: ApolloContracts = ApolloContracts {
            apollo_token: (Addr::unchecked("contract0"), 1),
        };
        // When init the contract
        let apollo_contracts: ApolloContracts = init_contracts(&mut app, OWNER);
        assert_eq!(expected, apollo_contracts, "we are testing apollo token init with {:?}", expected);
    }
}
