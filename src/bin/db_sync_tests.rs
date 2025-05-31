use std::io::Read;

#[tokio::main]
async fn main() {

    // load in test JSON objects
    let mut f = std::fs::File::open("test/test_objects.json").unwrap();
    let mut buff = [0 as u8; 1024];
    let n = f.read(&mut buff).unwrap();
    let j: serde_json::Value = serde_json::from_slice(&buff[0..n]).unwrap();

    // create database
    let db = laws::database::Database::new("./data").await;

    db.create_table(
        j["create_obj"].clone()
    ).await.unwrap();

    db.create_document(&"students".to_string(), j["doc_obj"].clone()).await.unwrap();

    println!("{}", db.read_db().await.unwrap());
    println!("{}", db.read_table(&"students".to_string()).await.unwrap());
    println!("{}", db.read_document(&"students".to_string(), j["query_obj"].clone()).await.unwrap());

    db.update_document(&"students".to_string(), j["update_obj"].clone()).await.unwrap();

    println!("{}", db.read_table(&"students".to_string()).await.unwrap());
    println!("{}", db.read_document(&"students".to_string(), j["query_obj"].clone()).await.unwrap());


    db.save().await;

}
