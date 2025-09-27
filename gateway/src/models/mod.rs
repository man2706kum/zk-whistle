use oximod::{
    _mongodb::bson::{DateTime, oid::ObjectId},
    Model,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrgType {
    Corporate,
    Government,
    NGO,
}

#[derive(Debug, Serialize, Deserialize, Model)]
#[db("zkWhistle")]
#[collection("proofs")]
pub struct Submission {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<ObjectId>,

    #[validate(includes = ".")]
    pub domain: String,

    #[validate(starts_with = "0x")]
    user_id: String,

    #[default(OrgType::Corporate)]
    pub org_type: OrgType,

    #[default(DateTime::now())]
    pub submission_date: DateTime,

    #[validate(non_empty)]
    pub title: String,

    #[validate(non_empty)]
    pub content: String,

    #[index(unique)]
    proof: ObjectId,

    #[default(false)]
    active: bool,

    ai_comment: String,

    cid: String,

    zk_pdf_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Model)]
#[db("zkWhistle")]
#[collection("verifications")]
pub struct Verification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub uuid: String,
    pub status: String,
    pub proof_hash: String,
}
