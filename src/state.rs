use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{IndexedMap, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Notification {
    pub sender: Addr,
    pub payload: String,
}

// this message is a broadcast msg
// think of it as a microblog
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Msg {
    // move to uuid later
    // pub uuid: String,
    pub body: String,
    pub mentions: Vec<Addr>,
    pub sender: Addr,
    // IPFS
    pub image_uri: Option<String>,
    pub identifier: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MsgInfoResponse {
    pub body: String,
    pub mentions: Vec<Addr>,
    pub sender: Addr,
    // IPFS
    pub image_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PrivateMsg {
    // move to uuid later
    // pub uuid: String,
    pub body: String,
    pub sender: Addr,
    // IPFS
    pub image_uri: Option<String>,
    pub identifier: u8,
}

// mapping of uuid to message
// pub const MESSAGES: Map<String, Msg> = Map::new("messages");

// this keeps a list of indexers
pub struct MsgIndexes<'a> {
    pub identifier: UniqueIndex<'a, U8Key, Msg>,
}

// IndexList is just boilerplate code for fetching a struct's indexes
impl<'a> IndexList<Msg> for MsgIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Msg>> + '_> {
        let v: Vec<&dyn Index<Msg>> = vec![&self.identifier];
        Box::new(v.into_iter())
    }
}

// msgs() is the storage access function.
pub fn msgs<'a>() -> IndexedMap<'a, &'a [u8], Msg, MsgIndexes<'a>> {
    let indexes = MsgIndexes {
        identifier: UniqueIndex::new(|d| U8Key::new(d.identifier), "msg_identifier"),
    };
    IndexedMap::new("messages", indexes)
}

// private msgs - need to be a multi-index
// i.e. index over private msgs, by recipient
