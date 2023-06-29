use soroban_sdk::{Env, Address};

use crate::types::{DataKey, Error, LockedOutBridge, Bridged, Hash};

fn has_node(env: &Env, node: Address) -> bool {
    env.storage().has(
        &DataKey::NodeSlot(node)
    )

}

pub(crate) fn verify_node(env: &Env, node: Address) -> Result<(), Error> {
    node.require_auth();
    if has_node(env, node) {
        Ok(())
    } else {
        Err(Error::NodeIdDoesntExist)
    }
}

pub(crate) fn write_object(env: &Env, node: Address, object: LockedOutBridge) {
    let hash = object.hash().clone();
    let key = Bridged(node, hash);

    env.storage().set(&key, &object);
}

pub(crate) fn read_object(env: &Env, node: Address, hash: Hash) -> LockedOutBridge {
    let key = Bridged(node, hash);
    
    env.storage().get(&key).unwrap().unwrap()
}
