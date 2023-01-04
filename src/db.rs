use mongodb::*;


pub async fn get_database() -> Result<Database, mongodb::error::Error> {

    let mongodb_uri =  "mongodb://mongo:RDeXpyBeNFsHH6E8UScK@containers-us-west-168.railway.app:6976";
    let client = Client::with_uri_str(mongodb_uri).await?;
    let db =  client.database("test");

    Ok(db)
    
}