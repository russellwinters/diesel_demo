mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use models::Players;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn upsert_players() -> Result<(), diesel::result::Error> {
    let connection = establish_connection();

    // TODO: run read_players_file, mapping over each row to create a player struct
    // TODO: Publ insert players from above into diesel
    let new_player = Players {
        id: 1,
        full_name: "John Smith".to_string(),
        last_name: "John".to_string(),
        first_name: "John".to_string(),
        is_active: "true".to_string(),
    };

    // diesel::insert_into(players)
    //     .values(&new_player)
    //     .on_conflict(id)
    //     .do_update()
    //     .set(&new_player)
    //     .execute(&connection)?;

    Ok(())
}

// TODO: make this read players file and return a csv parsed vector of all the rows
pub fn read_players_file() -> Result<Vec<Players>, std::io::Error> {
    use csv::Reader;
    use std::fs::File;

    let file = File::open("data/players.csv").expect("Failed to read players file");
    let mut rdr = Reader::from_reader(file);

    let results: Result<Vec<Players>, csv::Error> = rdr
        .records()
        .map(|result: Result<csv::StringRecord, csv::Error>| {
            let record = result.expect("Failed to get record");
            record.deserialize(None)
        })
        .collect();

    let players = results.expect("Players were not collected properly");

    for result in &players {
        let record = result;
        println!("{:?}", record);
    }

    Ok(players)
}

// TODO: figure this out
// pub fn get_players() -> Result<Vec<Players>, diesel::result::Error> {
//     use schema::players::dsl::*;

//     let mut connection = establish_connection();
//     let results = players
//         .load::<Players>(connection)
//         .expect("Failed to load players");

//     Ok(results)
// }
