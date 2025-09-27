use std::str::FromStr;

use crate::models::{OrgType, Submission, Verification};
use crate::routes::types::{ErrorResponse, Proof, ResponseSubmission, SubmissionResponse};
use futures::TryStreamExt;
use poem::http::StatusCode;
use poem::web::{Json, Query};
use poem::{IntoResponse, Result, handler};
use serde::{Deserialize, Serialize};

use oximod::_mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use oximod::{Model, get_global_client};

#[handler]
pub async fn submit_proof(res: Result<Json<Proof>>) -> Result<impl IntoResponse> {
    let result = res?;
 
    //check if proof is already verified
    let v = Verification::find_one(doc! {"uuid" : result.proof.to_string() }).await;

    if v.is_err() {
        return Ok(
            Json(serde_json::json!(ErrorResponse { error : String::from("Could not fetch proof") }))
                .with_status(StatusCode::INTERNAL_SERVER_ERROR)
                .into_response(),
        );
    }

    let o = v.unwrap();

    if o.is_none() {
        return Ok(Json(serde_json::json!(ErrorResponse { error : String::from("Proof not found") }))
            .with_status(StatusCode::BAD_REQUEST)
            .into_response());
    }

    let proof_id = ObjectId::from_str(o.unwrap()._id.unwrap().to_string().as_ref()).expect("proof");

    let submission = Submission::new()
        .active(true)
        .title(result.title.to_owned())
        .domain(result.domain.to_string())
        .user_id(result.user_id.to_owned().unwrap_or("0x".to_string()))
        .org_type(result.org_type.clone())
        .proof(proof_id)
        .content(result.content.to_owned());

    let submission_result = submission.save().await;

    match submission_result {
        Ok(submission_id) => {
            let response = SubmissionResponse {
                msg: "Submission received !".to_string(),
                submission_id: submission_id.to_hex(),
            };

            Ok(Json(serde_json::json!(response))
                .with_status(StatusCode::CREATED)
                .into_response())
        }
        Err(_) => Ok(
            Json(serde_json::json!(ErrorResponse { error : String::from("Could not submit proof") }))
                .with_status(StatusCode::INTERNAL_SERVER_ERROR)
                .into_response(),
        ),
    }
}

#[derive(Deserialize, Serialize)]
struct SubmissionParams {
    org_type: Option<OrgType>,
    offset: Option<u64>,
    page_size: Option<i64>,
}

#[handler]
pub async fn get_submissions(
    query: Result<Query<SubmissionParams>>,
) -> poem::Result<impl IntoResponse> {
    let result_query = query?;
    let skip = match result_query.offset {
        Some(v) => v,
        None => 0,
    };

    let limit = match result_query.page_size {
        Some(v) => v,
        None => 0,
    };

    let mut filter_query = doc! {};
    match &result_query.org_type {
        Some(v) => {
            filter_query.insert("org_type", oximod::_mongodb::bson::to_bson(v).unwrap());
        }
        None => {}
    }

    let c = get_global_client().unwrap();
    let collection: Collection<Submission> = c.database("zkWhistle").collection("proofs");
    let cursor_result = collection
        .find(filter_query)
        .skip(skip)
        .limit(limit)
        .await;

    if cursor_result.is_err() {
      return  Ok(
            Json(serde_json::json!(ErrorResponse { error : String::from("Unable to query") }))
                .with_status(StatusCode::INTERNAL_SERVER_ERROR)
                .into_response(),
        );
    }

    let mut cursor = cursor_result.unwrap();

    let mut res: Vec<ResponseSubmission> = vec![];
    while let Some(doc) = cursor.try_next().await.unwrap() {
        res.push(ResponseSubmission {
            title: doc.title,
            content: doc.content,
            domain: doc.domain,
            org_type: doc.org_type,
            submission_date: doc.submission_date.try_to_rfc3339_string().unwrap(),
        });
    }

    Ok(Json(serde_json::json!({ "submissions" : res}))
        .with_status(StatusCode::OK)
        .into_response())
}
