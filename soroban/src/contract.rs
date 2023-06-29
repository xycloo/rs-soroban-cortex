use soroban_sdk::{contractimpl, Env, BytesN, Address, Vec, vec};

use crate::{storage::{verify_node, write_object, read_object}, types::{Error, LockedOutBridge, DataKey, Hash}};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn set_node(env: Env, node: Address) {
        env.storage().set(&DataKey::NodeSlot(node.clone()), &true);
        let mut nodes: Vec<Address> = env.storage().get(&DataKey::Nodes)
            .unwrap_or(Ok(vec![&env]))
            .unwrap();
        nodes.push_back(node);
        env.storage().set(&DataKey::Nodes, &nodes)
    }

    pub fn oracle_write_object(env: Env, node: Address, payload: BytesN<80>) -> Result<(), Error> {
        // verify that the node is authorized to provide data to its slot
        verify_node(&env, node.clone())?;

        // deserialize the payload to store it.
        let object = LockedOutBridge::deserialize_from_bytes(&env, payload.to_array());

        // write object to storage.
        write_object(&env, node, object);

        Ok(())
    }

    pub fn collect_locked(env: Env, hash: Hash) {
        let nodes: Vec<Address> = env.storage().get(&DataKey::Nodes)
            .unwrap_or(Ok(vec![&env]))
            .unwrap();
        
        let mut aggregated = vec![&env];

        // todo: only if nodes length >= storage.minimum_nodes
        // todo: aggregate only when the node has submitted
        // maybe some reporting on which nodes are not provding data
        // would be cool. Probably emitted as an event.
        for node in nodes.iter_unchecked() {
            aggregated.push_back(read_object(&env, node, hash.clone()));
        }

        // here
        // if a minimum of nodes participates:
        //   nodes >= strage.min_participant_nodes
        
        // compare results in aggregated results
        // sort out the right message
        // if ratio of right message is above storage.min_ratio
        //   allow `recipient` to withdraw
        //   emit event to be read by nodes
        //   (the node which unlocks the funds on the other chain gets a small fee)
    }

    
}
