use sqlx::FromRow;
use serde::Serialize;

#[derive(FromRow, Serialize)]
pub struct CompetitionResponse{
    pub id: i64,
    pub name: String,
    pub logo: String,
    pub is_active: bool,
    pub is_complete: bool,
    pub region_id: i64,
    pub sort_order: i64,
}

#[derive(FromRow, Serialize)]
pub struct LeagueResponse {
    pub id: i64,
    pub name: String,
    pub logo: String,
    pub tournament_type_id: i64,
    pub locked_events: i64,
    pub competition_id: i64,
    pub competiton: Option<CompetitionResponse>
}