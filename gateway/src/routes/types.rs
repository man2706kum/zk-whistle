use serde::{Deserialize, Serialize};

use crate::models::OrgType;

#[derive(Deserialize, Serialize)]
pub struct Proof {
    pub proof: String,
    pub domain: String,
    pub user_id: Option<String>,
    pub title: String,
    pub content: String,
    pub org_type: OrgType,
    pub cid: Option<String>,
    pub ai_verification : Option<bool>,
    pub proof_content : Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SubmissionResponse {
    pub msg: String,
    pub submission_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseSubmission {
    pub title: String,
    pub content: String,
    pub org_type: OrgType,
    pub submission_date: String,
    pub domain: String,
}


#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error : String
}