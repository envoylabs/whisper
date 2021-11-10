use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // todo - admin DAO
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    PushNotification {
        sender: String,
        payload: String,
    },
    // incentivise and add read/pull receipt
    // this would be a client that pulls things off
    // and actions them
    // maybe with CW20
    PullNotification {
        sender: String,
        payload: String,
    },
    CreatePublicMsg {
        body: String,
        mentions: Vec<String>,
        creator: String,
        // IPFS
        image_uri: Option<String>,
    },
    UpdatePublicMsg {
        body: String,
        mentions: Vec<String>,
        // IPFS
        image_uri: Option<String>,
    },
    // can be deleted by OWNER || admin DAO
    DeletePublicMsg {
        // todo - use uuid
        identifier: u8,
    },
    // upvote
    YoloPublicMsg {
        // todo - use uuid
        identifier: u8,
    }, // upvote

    // naive impl:
    // just store on chain
    // but we need to gradually move away from storing this on chain
    // at all. At present needs more thought
    // as to how you would incentivise, e.g.
    // sender pays
    // plus a value on top
    // on read receipt, there is a payout
    // question - disappearing messages?
    // there's a bunch more that needs to be done on this
    // beyond simply a bank tx with encrypted memo
    CreatePrivateMsg {
        body: String,
        sender: String,
        recipient: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PullNotificationResponse {
    pub body: String,
    pub mentions: Vec<Addr>,
    pub sender: Addr,
    pub image_uri: Option<String>,
    pub identifier: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPublicMsg { identifier: u8 },
    GetPublicMsgsForAddr { address: String },
    GetMentionsForAddr { address: String },

    GetPrivateMsgsForAddr { address: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetPublicMsgResponse {
    pub body: String,
    pub mentions: Vec<Addr>,
    pub sender: Addr,
    pub image_uri: Option<String>,
    pub identifier: u8,
}

// This covers the list msgs case
// i.e. public, private and mentions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetMsgsResponse {
    pub messages: Vec<GetPublicMsgResponse>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetPrivateMsgResponse {
    pub body: String,
    pub sender: Addr,
}

// This covers the private list msgs case
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetPrivateMsgsResponse {
    pub messages: Vec<GetPrivateMsgResponse>,
}
