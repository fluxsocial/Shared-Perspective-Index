use chrono::{DateTime, Utc};
use hdk::prelude::*;

mod inputs;
mod utils;

use inputs::*;
use utils::err;

//TODO
// DNA needs spam protection since it is supposed to be public
// Validation on the linking of some shared perspective to some key to be sure that only the original key creator can create a link

/// Expression data this DNA is "hosting"
#[hdk_entry(id = "shared_perspective", visibility = "public")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignedSharedPerspective {
    data: SharedPerspective,
    author: Agent,
    timestamp: DateTime<Utc>,
    proof: ExpressionProof,
}

#[derive(Clone, SerializedBytes, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SharedPerspective {
    name: String,
    description: String,
    r#type: String,
    link_languages: Vec<String>,
    allowed_expression_languages: Vec<String>,
    required_expression_languages: Vec<String>,
}

// #[derive(Clone, SerializedBytes, Serialize, Deserialize, Debug)]
// pub enum SharingType  {
//     Broadcast,
//     Permissionless,
//     Permissioned,
//     Holochain
// }

#[derive(Clone, SerializedBytes, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateSharedPerspective {
    pub key: String,
    pub shared_perspective: SignedSharedPerspective,
}

entry_defs![SignedSharedPerspective::entry_def(), Path::entry_def()];

// Zome functions

/// Run function where zome is init'd by agent
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn index_shared_perspective(data: CreateSharedPerspective) -> ExternResult<SignedSharedPerspective> {
    let path = Path::from(data.key);
    path.ensure()?;

    create_entry(&data.shared_perspective)?;
    let expression_hash = hash_entry(&data.shared_perspective)?;
    create_link(path.hash()?, expression_hash, ())?;
    Ok(data.shared_perspective)
}

#[hdk_extern]
pub fn get_latest_shared_perspective(key: String) -> ExternResult<Option<SignedSharedPerspective>> {
    let path = Path::from(key);
    let mut links = get_links(path.hash()?, None)?
        .into_inner()
        .into_iter()
        .map(|link| match get(link.target, GetOptions::latest())? {
            Some(chunk) => Ok(Some(
                chunk
                    .entry()
                    .to_app_option::<SignedSharedPerspective>()?
                    .ok_or(err("Expected element to contain app entry data"))?,
            )),
            None => Ok(None),
        })
        .filter_map(|val| {
            if val.is_ok() {
                let val = val.unwrap();
                if val.is_some() {
                    Some(Ok(val.unwrap()))
                } else {
                    None
                }
            } else {
                Some(Err(val.err().unwrap()))
            }
        })
        .collect::<ExternResult<Vec<SignedSharedPerspective>>>()?;
    links.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
    Ok(links.first().cloned())
}

#[hdk_extern]
pub fn get_all_shared_perspectives(key: String) -> ExternResult<Vec<SignedSharedPerspective>> {
    let path = Path::from(key);
    let links = get_links(path.hash()?, None)?
        .into_inner()
        .into_iter()
        .map(|link| match get(link.target, GetOptions::latest())? {
            Some(chunk) => Ok(Some(
                chunk
                    .entry()
                    .to_app_option::<SignedSharedPerspective>()?
                    .ok_or(err("Expected element to contain app entry data"))?,
            )),
            None => Ok(None),
        })
        .filter_map(|val| {
            if val.is_ok() {
                let val = val.unwrap();
                if val.is_some() {
                    Some(Ok(val.unwrap()))
                } else {
                    None
                }
            } else {
                Some(Err(val.err().unwrap()))
            }
        })
        .collect::<ExternResult<Vec<SignedSharedPerspective>>>()?;
    Ok(links)
}

// #[hdk_extern]
// pub fn update_shared_perspective(data: CreateSharedPerspective) -> ExternResult<SharedPerspective> {
//     let path = Path::from(data.key);
//     path.ensure()?;

//     create_entry(&data.shared_perspective)?;
//     let expression_hash = hash_entry(&data.shared_perspective)?;
//     create_link(path.hash()?, expression_hash, ())?;
//     Ok(data.shared_perspective)
// }
