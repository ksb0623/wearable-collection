use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Binary, Coin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Multiple NFTs can linked to the target NFT and ownership is changed(locked)
    ComposeNft {
        target_token_id: String,
        parts_token_ids: Vec<String>,
    },
    
    /// Multiple NFTs can unlinked from the Target NFT and ownership returns to it's original state.
    DecomposeNft {
        target_token_id: String,
        parts_token_ids: Vec<String>,
    },

    /// All parts NFT of the target NFT are unlinked and ownership is returned to the original state.
    DecomposeAll {
        target_token_id: String,
    },

    /// Transfer is a base message to move a composed token to another account without triggering actions
    /// Only composed NFT with a dependency of 1 depth can be transfer
    TransferComposedNft { 
        recipient: String, 
        token_id: String 
    },
    
    /// Send is a base message to transfer a composed token to a contract and trigger an action
    /// on the receiving contract.
    /// Only composed NFT with a dependency of 1 depth can be send
    SendComposedNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },

    /// Sets address to send withdrawn fees to. Only owner can call this.
    SetWithdrawAddress { address: String },
    /// Removes the withdraw address, so fees are sent to the contract. Only owner can call this.
    RemoveWithdrawAddress {},
    /// Withdraw from the contract to the given address. Anyone can call this,
    /// which is okay since withdraw address has been set by owner.
    WithdrawFunds { amount: Coin },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ChildDependentTokenInfo { 
        token_id: String 
    },

    ParentDependentTokenInfo { 
        token_id: String 
    },

    GetWithdrawAddress {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct NftInfoResponse {
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// You can add any custom metadata here when you extend cw721-base
    pub extension: Option<Metadata>,
    /// Token ID for dependent NFTs
    pub dependent_tokens: Option<Vec<String>>,
}
