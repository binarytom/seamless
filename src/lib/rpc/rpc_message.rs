use serde::{Deserialize, Serialize};

/// rpc message definition
///
///
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum RpcMessage {
    ContentMessage { content: String },
    CommandMessage { command: RpcCommand },
    OtherMessage,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum RpcCommand {
    TestCommand,
}

#[cfg(test)]
mod test {
    use crate::lib::rpc::rpc_message::RpcMessage;
    #[test]
    fn serialize() {
        let content = "Hello World!".to_string();
        let rpc_message = RpcMessage::ContentMessage { content };
        let serialized = bincode::serialize(&rpc_message).unwrap();
        println!("{:?}", serialized);

        let bytes: Vec<u8> = vec![
            0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108,
            100, 33,
        ];
        assert_eq!(bytes, serialized);
    }

    #[test]
    fn deserialize() {
        let serialized: Vec<u8> = vec![
            0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108,
            100, 33,
        ];
        let deserized = bincode::deserialize::<RpcMessage>(&serialized).unwrap();

        let content = "Hello World!".to_string();
        let rpc_message = RpcMessage::ContentMessage { content };
        assert_eq!(rpc_message, deserized);
    }
}
