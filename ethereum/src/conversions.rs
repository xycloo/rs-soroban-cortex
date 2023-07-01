use multichain_core::node::{TryIntoMessage, NodeError};
use web3::types::Log;
pub struct LogWrap(pub Result<Log, web3::Error>);


impl TryIntoMessage for LogWrap {
    fn try_into(self) -> Result<[u8; 80], NodeError> {
        let mut bytes = [0_u8; 80];

        let data = self.0.unwrap().data.0; // TODO: error handling
        if data.len() != 80 {
            return Err(NodeError::Conversion);
        }
        bytes.copy_from_slice(data.as_slice());

        Ok(bytes)
    }
}
