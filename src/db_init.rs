use futures::TryStreamExt; // gives `.try_collect()` to turn a cursor into a Vec
use mongodb::bson::doc; // `doc!{}` builds a mongo document (a filter, a sort...)
use mongodb::{Client, Database};

use crate::models::{ScoreEntry, Unit};

const CHARACTERS: &str = "characters";
const MONSTERS: &str = "monsters";
const SCORES: &str = "scores";

pub struct Repo {
    db: Database,
}

impl Repo {
    pub async fn connect(uri: &str, db_name: &str) -> Result<Repo, Box<dyn std::error::Error>> {
        let client = Client::with_uri_str(uri).await?;
        let db = client.database(db_name);

        db.run_command(doc! { "ping": 1 }).await?;

        Ok(Repo { db })
    }

    pub async fn all_characters(&self) -> Result<Vec<Unit>, Box<dyn std::error::Error>> {
        let collection = self.db.collection::<Unit>(CHARACTERS);
        let cursor = collection.find(doc! {}).await?;
        let units: Vec<Unit> = cursor.try_collect().await?;
        Ok(units)
    }

    pub async fn all_monsters(&self) -> Result<Vec<Unit>, Box<dyn std::error::Error>> {
        let collection = self.db.collection::<Unit>(MONSTERS);
        let cursor = collection.find(doc! {}).await?;
        let units: Vec<Unit> = cursor.try_collect().await?;
        Ok(units)
    }

    pub async fn save_score(&self, entry: &ScoreEntry) -> Result<(), Box<dyn std::error::Error>> {
        let collection = self.db.collection::<ScoreEntry>(SCORES);
        collection.insert_one(entry.clone()).await?;
        Ok(())
    }

    // Get the `n` best scores, the biggest number of waves first.
    pub async fn top_scores(&self, n: i64) -> Result<Vec<ScoreEntry>, Box<dyn std::error::Error>> {
        let collection = self.db.collection::<ScoreEntry>(SCORES);
        let cursor = collection
            .find(doc! {})
            .sort(doc! { "waves": -1 })
            .limit(n)
            .await?;
        let scores: Vec<ScoreEntry> = cursor.try_collect().await?;
        Ok(scores)
    }
}
