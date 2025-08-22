use crate::oidc::Token;
use actix_web::dev::JsonBody;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::Responder;
use serde::Deserialize;
use serde::Serialize;
use sqlx::{Database, Executor};
use sqlx::FromRow;
use sqlx::Pool;
use sqlx::Postgres;
use std::error::Error;
use sqlx::error::BoxDynError;

const DEFAULT_PAGE_SIZE: usize = 100;

#[get("/tickets")]
pub async fn list_tickets(user: web::ReqData<Token>, options: web::Query<ListTicketOptions>, db: web::Data<Pool<Postgres>>) -> Result<impl Responder, Box<dyn Error>> {
    let num_records = options.num_records.unwrap_or(DEFAULT_PAGE_SIZE);

    let tickets = sqlx::query!(r##"SELECT ticket_id, date, title, priority as "priority: TicketPriority", registrant, status as "status: TicketStatus", assignee FROM ticket LIMIT $1"##, num_records as i64)
        .fetch_all(&**db)
        .await?;

    // log::debug!("{:#?}", );

    Ok(web::Json(tickets.into_iter()
        .filter_map(|i| Some(Ticket {
            ticket_id: i.ticket_id? as u64,
            date: i.date?.and_utc(),
            title: i.title?,
            registrant: i.registrant? as UserID,
            assignee: i.assignee.map(|i| i as UserID),
            priority: i.priority?,
            status: i.status?,
        }))
        .collect::<Vec<_>>()))
}

// #[post("/tickets")]
// pub async fn create_ticket(user: web::Data<Token>, ticket: web::JsonBody<TicketBuilder>) -> Result<()> {
//     Ok(())
// }

pub type UserID = u64;

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum TicketPriority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum TicketStatus {
    #[default]
    New,
    InProgress,
    Done,
    Cancelled,
    WontFix,
    Duplicate,
    Stale,
    Resolved,
}

impl sqlx::Decode<'_, Postgres> for TicketPriority {
    fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
        match value.as_str()? {
            "low" => Ok(TicketPriority::Low),
            "normal" => Ok(TicketPriority::Normal),
            "high" => Ok(TicketPriority::High),
            "critical" => Ok(TicketPriority::Critical),
            _ => Err(Box::new(sqlx::Error::Decode(format!("Invalid priority: {}", value.as_str()?).into()))),
        }
    }
}

impl sqlx::Decode<'_, Postgres> for TicketStatus {
    fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
        match value.as_str()? {
            "new" => Ok(TicketStatus::New),
            "in progress" => Ok(TicketStatus::InProgress),
            "done" => Ok(TicketStatus::Done),
            "cancelled" => Ok(TicketStatus::Cancelled),
            "wont_fix" => Ok(TicketStatus::WontFix),
            "duplicate" => Ok(TicketStatus::Duplicate),
            "stale" => Ok(TicketStatus::Stale),
            "resolved" => Ok(TicketStatus::Resolved),
            _ => Err(Box::new(sqlx::Error::Decode(format!("Invalid status: {}", value.as_str()?).into())))
        }
    }
}

#[derive(Deserialize)]
pub struct ListTicketOptions {
    owner: Option<UserID>,
    priority: Option<Vec<TicketPriority>>,
    title: Option<String>,
    comment: Option<String>,
    status: Option<String>,
    num_records: Option<usize>,
    offset: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct TicketList {}

#[derive(Serialize, Deserialize)]
pub struct TicketBuilder {
    title: String,
    registrant: Option<UserID>,
    priority: Option<TicketPriority>,
    status: Option<TicketStatus>,
    comments: Vec<String>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Ticket {
    ticket_id: u64,
    date: chrono::DateTime<chrono::Utc>,
    title: String,
    registrant: UserID,
    assignee: Option<UserID>,
    priority: TicketPriority,
    status: TicketStatus,
}
