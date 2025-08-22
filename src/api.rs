use std::error::Error;
use actix_web::{get, post, web, Responder};
use actix_web::dev::JsonBody;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Pool, Postgres};
use crate::oidc::Token;

const DEFAULT_PAGE_SIZE: usize = 100;

#[get("/tickets")]
pub async fn list_tickets(user: web::ReqData<Token>, options: web::Query<ListTicketOptions>, db: web::Data<Pool<Postgres>>) -> Result<impl Responder, Box<dyn Error>> {
    let num_records = options.num_records.unwrap_or(DEFAULT_PAGE_SIZE);

    let tickets = sqlx::query!("SELECT * FROM tickets LIMIT $1", num_records as i64)
        .fetch_all(&**db)
        .await?;

    Ok(tickets
        .into_iter()
        .map(Ticket::from)
        .collect())
}

// #[post("/tickets")]
// pub async fn create_ticket(user: web::Data<Token>, ticket: web::JsonBody<TicketBuilder>) -> Result<()> {
//     Ok(())
// }

pub type UserID = String;

#[derive(Default, Serialize, Deserialize)]
pub enum TicketPriority {
    Low,
    #[default]
    Normal,
    High,
    Critical
}

#[derive(Default, Serialize, Deserialize)]
pub enum TicketStatus {
    #[default]
    New,
    InProgress,
    Done,
    Cancelled,
    WontFix,
    Duplicate,
    Stale,
    Resolved
}

#[derive(Deserialize)]
pub struct ListTicketOptions {
    owner: Option<UserID>,
    priority: Option<Vec<TicketPriority>>,
    title: Option<String>,
    comment: Option<String>,
    status: Option<String>,
    num_records: Option<usize>,
    offset: Option<usize>
}

#[derive(Serialize, Deserialize)]
pub struct TicketList {

}

#[derive(Serialize, Deserialize)]
pub struct TicketBuilder {
    title: String,
    registrant: Option<UserID>,
    priority: Option<TicketPriority>,
    status: Option<TicketStatus>,
    comments: Vec<String>
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Ticket {
    id: String,
    title: String,
    registrant: UserID,
    date: chrono::DateTime<chrono::Utc>,
    priority: TicketPriority,
    status: TicketStatus,
    comments: Vec<String>
}