#[macro_use]
extern crate rocket;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use yaml_rust::YamlLoader;

use std::collections::HashMap;

use mongodb::{
    bson::{self, doc, Document},
    options::FindOptions,
    options::{FindOneOptions},
    options::FindOneAndDeleteOptions,
    Client,
};
use rocket::{fs::FileServer, http::Status, response::content};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    uid: String,
    username: String,
    stocks: HashMap<String, i32>,
    bal: f64,
    rank: i32,
    pfp: String,
    inv: Vec<String>,
    equipped: Vec<String>,
}

#[get("/users")]
/// `users()` is an async function that returns a `content::RawJson<std::string::String>` type
///
/// Returns:
///
/// A JSON array of all the users in the database.
async fn users() -> content::RawJson<std::string::String> {
    let client = get_mongo_client().await;
    let db = client.database("users");
    let collection = db.collection::<Document>("users");
    let filter = doc! { "deleted": false };
    let options = FindOptions::builder().build();
    let mut cursor = collection.find(filter, options).await.unwrap();
    let mut json_array = "[".to_string();
    while cursor.advance().await.expect("well shit") {
        let cuser = cursor.deserialize_current().unwrap();
        let mut json_form = serde_json::to_string(&cuser).unwrap();
        json_form += ",";
        json_array.push_str(&json_form);
    }
    json_array.pop();
    if json_array.chars().count() > 1 {
        json_array.push_str("]");
    }
    content::RawJson(json_array)
}

#[get("/user/<uid>")]
/// It takes a user id, connects to the database, finds the user with that id, and returns the user as a
/// JSON object
///
/// Arguments:
///
/// * `uid`: The user's unique ID.
///
/// Returns:
///
/// A JSON object
async fn user(uid: String) -> content::RawJson<std::string::String> {
    let client = get_mongo_client().await;
    let db = client.database("users");
    let collection = db.collection::<Document>("users");
    let filter = doc! { "UID": uid };
    let options = FindOneOptions::builder().build();
    let cursor = collection.find_one(filter, options).await.unwrap();
    let cuser = cursor.unwrap();
    let json_form = serde_json::to_string(&cuser).unwrap();
    content::RawJson(json_form)
}

#[post("/user/<pword>", format = "application/json", data = "<user>")]
/// It takes a JSON string, converts it to a BSON document, and inserts it into the database
///
/// Arguments:
///
/// * `user`: The user's username
async fn create_user(user: String, pword: String) -> Status {
    if pword != api_password() {
        return Status::Unauthorized;
    }
    let client = get_mongo_client().await;
    let db = client.database("users");
    let collection = db.collection::<Document>("users");
    let v: Value = serde_json::from_str(&user).unwrap();
    let doc = match bson::to_bson(&v) {
        Ok(bson::Bson::Document(doc)) => doc,
        _ => panic!("Error converting to BSON"),
    };
    collection.insert_one(doc, None).await.unwrap();
    Status::Ok
}

#[get("/userexists/<uid>")]
/// It takes a user id, connects to the database, finds the user with that id, and returns a boolean
/// indicating whether or not the user exists
///
/// Arguments:
///
/// * `uid`: The user's unique ID.
///
/// Returns:
///
/// A boolean indicating whether or not the user exists.
async fn userexists(uid: String) -> content::RawJson<std::string::String> {
    let client = get_mongo_client().await;
    let db = client.database("users");
    let collection = db.collection::<Document>("users");
    let filter = doc! { "UID": uid };
    let options = FindOneOptions::builder().build();
    let cursor = collection.find_one(filter, options).await.unwrap();
    let json_form = serde_json::to_string(&cursor).unwrap();
    content::RawJson((json_form != "null").to_string())
}


#[delete("/deleteuser/<uid>/<pword>")]
/// It takes a user id and a password, connects to the database, finds the user with that id, and
/// deletes the user
///
/// Arguments:
///
/// * `uid`: The user's unique ID.
/// * `pword`: The user's password.
///
/// Returns:
/// * Status
async fn deleteuser(uid: String, pword: String) -> Status {
    if pword != api_password() {
        return Status::Unauthorized;
    }
    let client = get_mongo_client().await;
    let db = client.database("users");
    let collection = db.collection::<Document>("users");
    let filter = doc! { "UID": uid };
    let options = FindOneAndDeleteOptions::builder().build();
    collection
        .find_one_and_delete(filter, options)
        .await
        .unwrap();
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("src/site/dist"))
        .mount(
            "/api/v1",
            routes![users, user, create_user, deleteuser, userexists],
        )
}

async fn get_mongo_client() -> Client {
    // Read and load the config.yaml file and then use the values to connect to the database.
    let filecontent = std::fs::read_to_string("config.yaml").unwrap();
    let docs = YamlLoader::load_from_str(filecontent.as_str()).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    let client = Client::with_uri_str(
        format!(
            "mongodb://{}:{}@{}:{}",
            doc["username"].as_str().unwrap(),
            doc["password"].as_str().unwrap(),
            doc["host"].as_str().unwrap(),
            doc["dbport"].as_i64().unwrap()
        )
        .as_str(),
    )
    .await
    .expect("Connection failed");
    client
}

fn api_password() -> String {
    let filecontent = std::fs::read_to_string("config.yaml").unwrap();
    let docs = YamlLoader::load_from_str(filecontent.as_str()).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    doc["api_password"].as_str().unwrap().to_string()
}
