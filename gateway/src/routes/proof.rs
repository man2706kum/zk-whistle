
use std::str::FromStr;
use crate::models::{OrgType, Submission, Verification};
use crate::routes::types::{ErrorResponse, Proof, ResponseSubmission, SubmissionResponse};
use futures::TryStreamExt;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use oximod::_mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use oximod::{Model, get_global_client};

pub async fn submit_proof(payload: web::Json<Proof>) -> impl Responder {
    let result = payload.into_inner();
    //check if proof is already verified
    let v = Verification::find_one(doc! {"uuid" : result.proof.to_string() }).await;

    if v.is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: String::from("Could not fetch proof")
        });
    }

    let o = v.unwrap();

    if o.is_none() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: String::from("Proof not found")
        });
    }

    let proof_id = ObjectId::from_str(o.unwrap()._id.unwrap().to_string().as_ref()).expect("proof");

    let submission = Submission::new()
        .active(true)
        .title(result.title.to_owned())
        .domain(result.domain.to_string())
        .user_id(result.user_id.to_owned().unwrap_or("0x".to_string()))
        .org_type(result.org_type.clone())
        .proof(proof_id)
        .content(result.content.to_owned())
        .cid(result.cid.to_owned().unwrap_or("".to_owned()));

    let submission_result = submission.save().await;

    match submission_result {
        Ok(submission_id) => {
            let response = SubmissionResponse {
                msg: "Submission received !".to_string(),
                submission_id: submission_id.to_hex(),
            };
            HttpResponse::Created().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: String::from("Could not submit proof")
        }),
    }
}


#[derive(Deserialize, Serialize)]
pub struct SubmissionParams {
    pub org_type: Option<OrgType>,
    pub offset: Option<u64>,
    pub page_size: Option<i64>,
}

pub async fn get_submissions(query: web::Query<SubmissionParams>) -> impl Responder {
    let result_query = query.into_inner();
    let skip = result_query.offset.unwrap_or(0);
    let limit = result_query.page_size.unwrap_or(0);

    let mut filter_query = doc! {};
    if let Some(v) = &result_query.org_type {
        filter_query.insert("org_type", oximod::_mongodb::bson::to_bson(v).unwrap());
    }

    let c = get_global_client().unwrap();
    let collection: Collection<Submission> = c.database("zkWhistle").collection("proofs");
    let cursor_result = collection.find(filter_query).skip(skip).limit(limit).await;

    if cursor_result.is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: String::from("Unable to query")
        });
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

    HttpResponse::Ok().json(serde_json::json!({ "submissions" : res }))
}
