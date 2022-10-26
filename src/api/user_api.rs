use crate::{models::user_model::User};
use rocket::{http::Status, serde::json::Json};

#[post("/user", data = "<new_user>")]
pub fn create_user(new_user: Json<User>) -> Result<Json<String>, Status> {
    // Error test
    if new_user.name == "error" {
        return Err(Status::InternalServerError);
    }

    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    // let result = create_user(data);
    
    let result = Json(String::from("User created successfully"));
        
    Ok(result)
}

#[get("/user/<path>")]
pub fn get_user(path: String) -> Result<Json<User>, Status> {
    let id = path;

    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let user = User {
        id: Some(1),
        name: String::from("Lucas Martins"),
        location: String::from("Brazil"),
        title: String::from("Software Engineer"),
    };

    Ok(Json(user))
}