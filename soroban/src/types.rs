use soroban_sdk::{contracttype, BytesN, Address, contracterror};

use crate::contract;

pub(crate) type Hash = BytesN<32>;

#[derive(Copy, Clone)]
#[repr(u32)]
#[contracterror]
pub enum Error {
    NodeIdDoesntExist = 0,
    NotEnoughNodes = 1,
    InvalidPreimage = 2,
    VoteDoesntFitRatio = 3,
}

#[contracttype]
pub struct Bridged(pub Address, pub Hash);

#[contracttype]
pub(crate) enum DataKey {
    Settings,
    Nodes,
    NodeSlot(Address),
    Bridged(Bridged)
}

#[contracttype]
pub(crate) struct Settings {
    /// minimum number of nodes which have provided at least the correct hash.
    /// the nodes who have provided another hash will still enter storage but under
    /// a different entry slot, and won't have enough [`minimum_participating`] nodes
    /// for their locked balance to be collected.
    pub minimum_participating_nodes: u32,
    
    pub minimum_approve_ratio: u32
}

impl Settings {
    pub(crate) fn new(minimum_participating_nodes: u32, minimum_approve_ratio: u32) -> Self {
        Self { 
            minimum_participating_nodes, 
            minimum_approve_ratio 
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[contracttype]
pub struct LockedOutBridge {
    hash: Hash,
    recipient: Address,
    amount: i128,
}

impl LockedOutBridge {
    pub(crate) fn hash(&self) -> &Hash {
        &self.hash
    }
}

mod deser {
    use soroban_sdk::{Env, BytesN, Address};

    use super::LockedOutBridge;

    fn bytes_to_i128(bytes: [u8; 16]) -> i128 {
        let mut result: i128 = 0;

        for i in (0..16).rev() {
            result <<= 8;
            result |= bytes[i] as i128;
        }

        // Check if the original value was negative
        if bytes[0] & 0x80 != 0 {
            result = !result + 1;
            result *= -1;
        }

        result
    }

    impl LockedOutBridge {
        pub(crate) fn deserialize_from_bytes(env: &Env, bytes: [u8; 80]) -> Self {
            let mut hash: [u8; 32] = [0; 32];
            let mut address: [u8; 32] = [0; 32];
            let mut amount: [u8; 16] = [0; 16];

            hash.copy_from_slice(&bytes[0..32]);
            address.copy_from_slice(&bytes[32..64]);
            amount.copy_from_slice(&bytes[64..80]);

            Self { 
                hash: BytesN::from_array(env, &hash), 
                recipient: Address::from_contract_id(
                    &BytesN::from_array(env, &address)
                ), 
                amount: bytes_to_i128(amount)
            }
        }
    }
}
