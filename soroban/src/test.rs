mod nodes {
    use soroban_sdk::{Env, Address, testutils::Address as _, BytesN};

    #[test]
    fn set_object() {
        let env = Env::default();
        env.mock_all_auths();

        let contract = env.register_contract(None, crate::contract::Contract);
        let client = crate::contract::ContractClient::new(&env, &contract);

        let oracle = Address::random(&env);

        client.set_node(&oracle);
        client.oracle_write_object(&oracle, &BytesN::from_array(&env, &[0;80]));
    }
}