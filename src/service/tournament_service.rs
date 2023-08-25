use crate::{repository::league_repository::LeagueRepository, data::league_response::LeagueResponse};
use sqlx::Error;

pub struct TournamentService;

impl TournamentService {
    pub async fn get_tournaments() -> Result<Vec<LeagueResponse>, Error> {
    let tournaments = LeagueRepository::fetch_incomplete_leagues().await?;

    Ok(tournaments)
    }
}