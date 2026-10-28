use futures::TryStreamExt; // gives `.try_collect()` to turn a cursor into a Vec
use mongodb::bson::doc; // `doc!{}` builds a mongo document (a filter, a sort...)
use mongodb::{Client, Database};

use crate::models::{ScoreEntry, Unit};

const CHARACTERS: &str = "characters";
const MONSTERS: &str = "monsters";
const SCORES: &str = "scores";

pub struct Repo {
    db: Database,
    rt: tokio::runtime::Runtime,
}

type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

impl Repo {
    pub fn connect_from_env() -> DbResult<Repo> {
        dotenvy::dotenv().ok();
        let mongo_uri = std::env::var("MONGODB_URI")?;
        let mongo_db = std::env::var("MONGODB_DB")?;
        Repo::connect(&mongo_uri, &mongo_db)
    }

    pub fn connect(uri: &str, db_name: &str) -> DbResult<Repo> {
        let rt = tokio::runtime::Runtime::new()?;
        let db = rt.block_on(connect_async(uri, db_name))?;
        Ok(Repo { db, rt })
    }

    pub fn get_valid_characters(&self, min: usize) -> DbResult<Vec<Unit>> {
        let heroes = self.rt.block_on(self.fetch_units(CHARACTERS))?;
        if heroes.len() < min {
            return Err(format!("not enough heroes in the database (need at least {min})").into());
        }
        Ok(heroes)
    }

    pub fn get_valid_monsters(&self) -> DbResult<Vec<Unit>> {
        let monsters = self.rt.block_on(self.fetch_units(MONSTERS))?;
        if monsters.is_empty() {
            return Err("no monster in the database".into());
        }
        Ok(monsters)
    }

    pub fn save_score(&self, entry: &ScoreEntry) -> DbResult<()> {
        self.rt.block_on(self.insert_score(entry))
    }

    pub fn top_scores(&self, n: i64) -> DbResult<Vec<ScoreEntry>> {
        self.rt.block_on(self.fetch_top_scores(n))
    }

    async fn fetch_units(&self, collection_name: &str) -> DbResult<Vec<Unit>> {
        let collection = self.db.collection::<Unit>(collection_name);
        let cursor = collection.find(doc! {}).await?;
        let units: Vec<Unit> = cursor.try_collect().await?;
        Ok(units)
    }

    async fn insert_score(&self, entry: &ScoreEntry) -> DbResult<()> {
        let collection = self.db.collection::<ScoreEntry>(SCORES);
        collection.insert_one(entry.clone()).await?;
        Ok(())
    }

    // sort: -1 = descending (biggest first), limit: keep only the first n
    async fn fetch_top_scores(&self, n: i64) -> DbResult<Vec<ScoreEntry>> {
        let collection = self.db.collection::<ScoreEntry>(SCORES);
        let cursor = collection.find(doc! {}).sort(doc! { "waves": -1 }).limit(n).await?;
        let scores: Vec<ScoreEntry> = cursor.try_collect().await?;
        Ok(scores)
    }
}

// Not a method: it runs before the Repo exists.
async fn connect_async(uri: &str, db_name: &str) -> DbResult<Database> {
    let client = Client::with_uri_str(uri).await?;
    let db = client.database(db_name);
    // `with_uri_str` does not really check that the server answers, so we ping it
    db.run_command(doc! { "ping": 1 }).await?;
    Ok(db)
}
