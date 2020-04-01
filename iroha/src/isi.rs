use crate::{account, asset, domain, peer, wsv::WorldStateView};
use parity_scale_codec::{Decode, Encode};

/// Identification of an Iroha's entites. Consists of Entity's name and Domain's name.
///
/// # Example
///
/// ```
/// use iroha::isi::Id;
///
/// let id = Id::new("gold", "mine");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash, Encode, Decode)]
pub struct Id(pub String, pub String);

impl Id {
    pub fn new(entity_name: &str, domain_name: &str) -> Self {
        Id(entity_name.to_string(), domain_name.to_string())
    }
}

/// Iroha provides a library of smart contracts called **I**roha **S**pecial **I**nstructions (ISI).
/// To execute logic on the ledger, these smart contracts can be invoked via either transactions
/// or registered event listeners.
/// This trait represents API which every ISI should be aligned with.
pub trait Instruction {
    /// To execute the instruction this method implementation supplied with a mutable reference
    /// to `WorldStateView`. It's responsibility of the instruction to keep `WSV` in a consistent
    /// state and return `Err` in case of errors.
    fn execute(&self, world_state_view: &mut WorldStateView) -> Result<(), String>;
}

///
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub enum Contract {
    AddSignatory(account::isi::AddSignatory),
    AppendRole(account::isi::AppendRole),
    CreateAccount(account::isi::CreateAccount),
    CreateRole(account::isi::CreateRole),
    AddAssetQuantity(asset::isi::AddAssetQuantity),
    TransferAsset(asset::isi::TransferAsset),
    CreateAsset(asset::isi::CreateAsset),
    CreateDomain(domain::isi::CreateDomain),
    AddPeer(peer::isi::AddPeer),
}

impl Contract {
    pub fn invoke(&self, world_state_view: &mut WorldStateView) -> Result<(), String> {
        use Contract::*;
        match self {
            AddAssetQuantity(instruction) => instruction.execute(world_state_view),
            CreateAccount(instruction) => instruction.execute(world_state_view),
            CreateDomain(instruction) => instruction.execute(world_state_view),
            TransferAsset(instruction) => instruction.execute(world_state_view),
            _ => Err("Instruction is not supported yet.".to_string()),
        }
    }
}

impl std::convert::From<&Contract> for Vec<u8> {
    fn from(command_payload: &Contract) -> Self {
        command_payload.encode()
    }
}

impl std::convert::From<Vec<u8>> for Contract {
    fn from(command_payload: Vec<u8>) -> Self {
        Contract::decode(&mut command_payload.as_slice()).expect("Failed to deserialize payload.")
    }
}

pub enum Relation {
    /// Belongs to account with defined identification.
    /// For example we can fill a map of accounts to assets by this relation.
    OwnedBy(Id),
    GoingTo(Id),
}

/// This trait should be implemented for commands with `account_id` field.
/// Marking your command with `impl` of this trait you provide an ability
/// to retrieve information about relation to an account.
pub trait Property {
    fn relations(&self) -> Vec<Relation>;
}

impl Property for Contract {
    //TODO: implement
    fn relations(&self) -> Vec<Relation> {
        use Relation::*;
        match self {
            Contract::TransferAsset(instruction) => {
                let instruction = instruction.clone();
                vec![
                    GoingTo(instruction.destination_account_id),
                    OwnedBy(instruction.source_account_id),
                ]
            }
            _ => Vec::new(),
        }
    }
}

pub trait Assetibility {
    fn assets(&self) -> Vec<Id>;
}

impl Assetibility for Contract {
    //TODO: implement
    fn assets(&self) -> Vec<Id> {
        match self {
            Contract::TransferAsset(instruction) => {
                let instruction = instruction.clone();
                vec![instruction.asset_id]
            }
            _ => Vec::new(),
        }
    }
}