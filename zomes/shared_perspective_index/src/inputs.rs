use hdk::prelude::*;

#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct ExpressionProof {
    pub signature: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, SerializedBytes)]
pub struct Agent {
    pub did: String,
    pub name: Option<String>,
    pub email: Option<String>,
}
