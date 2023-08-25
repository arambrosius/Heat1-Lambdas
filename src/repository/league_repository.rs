use sqlx::{Error, Row};

use crate::data::{
    data_client::DataClient,
    league_response::{CompetitionResponse, LeagueResponse},
};

pub struct LeagueRepository;

impl LeagueRepository {
    pub async fn fetch_incomplete_leagues() -> Result<Vec<LeagueResponse>, Error> {
        let pool = DataClient::connect().await?;

        let res = sqlx::query(
            "
                SELECT 
                    tournament.id as tournament_id, 
                    tournament.name as tournament_name, 
                    tournament.logo as tournament_logo, 
                    tournament.tournament_type_id, 
                    tournament.competition_id, 
                    tournament.locked_events,
                    competition.id as competition_id, 
                    competition.name as competition_name, 
                    competition.logo as competition_logo, 
                    competition.is_active::bigint as is_active, 
                    competition.is_complete::bigint as is_complete,
                    competition.region_id, 
                    competition.sort_order
                FROM 
                    tournament
                    JOIN competition on tournament.competition_id = competition.id
                WHERE
                    competition.is_complete = '1'
                ",
        )
        .fetch_all(&pool)
        .await?;

        let t = res
            .iter()
            .map(|row| {
                LeagueResponse {
                    id: row.get("tournament_id"),
                    name: row.get("tournament_name"),
                    competition_id: row.get("competition_id"),
                    logo: row.get("tournament_logo"),
                    tournament_type_id: row.get("tournament_type_id"),
                    locked_events: row.get("locked_events"),
                    competiton: Some(CompetitionResponse {
                        id: row.get("competition_id"),
                        name: row.get("competition_name"),
                        logo: row.get("competition_logo"),
                        is_active: row.get::<i64, _>("is_active") == 1,
                        is_complete: row.get::<i64, _>("is_complete") == 1,
                        region_id: row.get("region_id"),
                        sort_order: row.get("sort_order"),
                    }),
                }
            })
            .collect();

        // let tournament_id: i64 = res.get("id");
        // let id = tournament_id as u64;
        // println!("id: {}", id);

        //print the results of the query of type pgRow
        // println!("id: {}", pgRow.get("id"));

        // let sql = "SELECT tournament.id FROM tournament JOIN competition on tournament.competition_id = competition.id WHERE competition.is_complete = false ORDER BY competition.sort_order";
        // let stmt = client
        //     .prepare("SELECT tournament.id FROM tournament WHERE UPPER(username) = $1::TEXT")
        //     .await?;
        // let rows = client.query(&stmt, &[]).await.map(
        //     |row| row.into().collect::<Vec<Tournament>>()
        // )?;

        // if rows.len() == 0 {
        //     println!("No rows returned from query");
        // }
        // let tournaments: Vec<Tournament> = rows.map();

        return Ok(t);
    }
}
