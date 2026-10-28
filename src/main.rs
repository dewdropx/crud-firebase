use std::collections::HashMap; // to store all users
use firebase_rs::*; // firebase realtime database
use serde::{Deserialize, Serialize}; // json serialize and deserialize

// user struct
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

// response struct
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

// set up tokio run time for asynchronous main function
#[tokio::main]
async fn main() {

    // define a hard coded user
    let user = User {
        name: "Arjun Mukherjee".to_string(),
        age: 20,
        email: "arjun@example.com".to_string(),
    };

    // establish connection to your firebase database
    let firebase = Firebase::new("https://crud-rust-firebase-default-rtdb.firebaseio.com/").unwrap();

    // call CRUD functions
    let response = set_user(&firebase, &user).await;
    let mut user = get_user(&firebase, &response.name).await;
    println!("{:?}", user);

    let users = get_users(&firebase).await;
    println!("{:?}", users);

    user.email = "updated@example.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await;
    println!("{:?}", updated_user);

    delete_user(&firebase, &response.name).await;
}

// create
async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let _users = firebase.set(user).await;
    return string_to_response(&_users.unwrap().data);
}

// read all
async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let _users = firebase.get::<HashMap<String, User>>().await;
    return _users.unwrap();
}

// read
async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(id);
    let _user = firebase.get::<User>().await;
    return _user.unwrap();
}

// update
async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(id);
    let _user = firebase.update(user).await;
    return string_to_user(&_user.unwrap().data);
}

// delete
async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(id);
    
    match firebase.delete().await {
        Ok(_) => println!("user deleted successfully"),
        Err(e) => println!("failed to delete user: {:?}", e),
    }
}

// convert a string to response struct
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

// convert a string to user struct
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}