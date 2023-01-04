use mongodb::*;


pub async fn get_database() -> Result<Database, mongodb::error::Error> {

    let mongodb_uri =  ""; // Your MongoDB URI here
    let client = Client::with_uri_str(mongodb_uri).await?;
    let db =  client.database("test");

    Ok(db)
    
}