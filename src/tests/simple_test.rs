#[test]
fn it_works() {
    assert_eq!(4, 4);
}


use crate::firestore_raw::firestore_raw::FirestoreRawBuilder;
use tokio;
#[tokio::test]
async fn construct_firestore_raw() {
    let firestore_builder = FirestoreRawBuilder::default();
    let firestore = firestore_builder.build();
    let resulting_text = firestore.get_document("https://firestore.googleapis.com/v1/projects/eisverygoodletter-discord-bot/databases/(default)/documents/discord_bot".into()).await;
}

use crate::firestore::document_container::DocumentContainer;
use crate::firestore::collection_container::CollectionContainer;
use crate::firestore::structs::FirestoreBuilder;
use crate::firestore::raw_data::RawDataContainer;
use serde_json::Value;
#[test]
fn test_firestore_get_url() {
    let mut firestore_builder = FirestoreBuilder::new();
    firestore_builder.from_url("somethingsomething/".to_string());
    // generate the firestore struct
    let firestore = firestore_builder.build();
    // check the get url
    assert_eq!(firestore.get_url(), "somethingsomething/".to_string());
    // 1 level recursion
    let citydoc = firestore.get_collection("cities/".to_string());
    assert_eq!(citydoc.get_url(), "somethingsomething/cities/".to_string());
    // 2 level recursion
    let avenue_document = citydoc.get_document("avenueavenue/".to_string());
    assert_eq!(avenue_document.get_url(), "somethingsomething/cities/avenueavenue/".to_string());
    // 3 level recursion
    let block_collection = avenue_document.get_collection("blocks/".to_string());
    assert_eq!(block_collection.get_url(), "somethingsomething/cities/avenueavenue/blocks/".to_string());
    // separate collection from firestore
    let town_collection = firestore.get_collection("towns/".to_string());
    // town url is accessible
    assert_eq!(town_collection.get_url(), "somethingsomething/towns/".to_string());
    // avenue document is still accessible
    assert_eq!(avenue_document.get_url(), "somethingsomething/cities/avenueavenue/".to_string());
}

#[tokio::test]
async fn test_custom_db_get() {
    let mut firestore_builder = FirestoreBuilder::new();
    let firestore = firestore_builder.from_env_basic().build();
    let testing_collection = firestore.get_collection("firestore-rest/".to_string());
    //println!("{:?}", testing_collection.get_raw_string().await);
    let test_file_1 = testing_collection.get_document("test file 1".to_string());
    println!("{:?}", test_file_1.get_complete_json().await.unwrap().to_string());
    let json_value: Value = test_file_1.get_complete_json().await.unwrap();
    assert_eq!(json_value["fields"]["value 1"]["stringValue"].as_str().unwrap(), "wow".to_string());
}