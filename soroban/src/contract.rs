use soroban_sdk::{contractimpl, Env, BytesN, Address, Vec, vec, Map, Bytes};

use crate::{storage::{verify_node, write_object, read_object, write_settings, read_settings}, types::{Error, LockedOutBridge, DataKey, Hash, Settings}};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn set_settings(env: Env, minimum_participating_nodes: u32, minimum_approve_ratio: u32) {
        // todo: auth here
        
        let settings = Settings::new(minimum_participating_nodes, minimum_approve_ratio);
        write_settings(&env, settings);
    }
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

    pub fn collect_locked(env: Env, hash: Hash, hash_preimage: Bytes) -> Result<(), Error> {
        let nodes: Vec<Address> = env.storage().get(&DataKey::Nodes)
            .unwrap_or(Ok(vec![&env]))
            .unwrap();
        
        let mut aggregated = vec![&env];

        // todo: only if nodes length >= storage.minimum_nodes
        // todo: aggregate only when the node has submitted
        // maybe some reporting on which nodes are not provding data
        // would be cool. Probably emitted as an event.
        for node in nodes.iter_unchecked() {
            if let Some(object) = read_object(&env, node, hash.clone()) {
               aggregated.push_back(object);
            }
        }
        
        let settings = read_settings(&env);
        
        if aggregated.len() < settings.minimum_participating_nodes {
            return Err(Error::NotEnoughNodes)
        }
        
        let mut votes: Map<LockedOutBridge, u32> = Map::new(&env);
        for entry in aggregated.iter_unchecked() {
            if let Some(Ok(x)) = votes.get(entry.clone()) {
                votes.set(entry, x + 1)
            } else {
                votes.set(entry, 1)
            }
        }
        
        let winning_vote = votes.clone().into_iter_unchecked().max_by_key(|(_, v)| *v).map(|(k, _)| k);
        if winning_vote.is_none() {
            return Err(Error::VoteDoesntFitRatio);
        }

        let winning_vote = winning_vote.unwrap();

        if aggregated.len() > votes.get(winning_vote).unwrap().unwrap() {
            return Err(Error::VoteDoesntFitRatio);
        }

        if hash != env.crypto().sha256(&hash_preimage) {
            return Err(Error::InvalidPreimage);
        }

        // here
        // transfer amount to recipient

        Ok(())
    }

    // todo
    // redeem locked tokens to contract if a certain amount of time has passed.
    // swap initiator still doesn't loose their funds.
    
}
