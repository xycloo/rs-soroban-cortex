mod nodes {
    use soroban_sdk::{Env, Address, testutils::Address as _, BytesN};

    #[test]
    fn set_object() {
        let env = Env::default();
        env.mock_all_auths();

        let contract = env.register_contract(None, crate::contract::Contract);
        let client = crate::contract::ContractClient::new(&env, &contract);

        let oracle = Address::random(&env);
        let oracle_1 = Address::random(&env);
        let oracle_2 = Address::random(&env);

        client.set_settings(&2, &51);

        client.set_node(&oracle);
        client.set_node(&oracle_1);
        client.set_node(&oracle_2);

        client.oracle_write_object(&oracle, &BytesN::from_array(&env, &[0;80]));
        client.oracle_write_object(&oracle_1, &BytesN::from_array(&env, &[0;80]));
        client.oracle_write_object(&oracle_2, &BytesN::from_array(&env, &[2;80]));
        
        // extern crate std;
        // std::println!("{:?}", client.collect_locked(&BytesN::from_array(&env, &[0;32])));
    }
}