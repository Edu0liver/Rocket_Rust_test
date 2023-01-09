
pub mod delay {
    use rocket::tokio::time::{sleep, Duration};
    use rocket::serde::{Deserialize, Serialize, json::Json};

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct User {
        name: String,
        email: String
    }

    #[get("/<seconds>")]
    pub async fn delay_route(seconds: u64) -> String {
        sleep(Duration::from_secs(seconds)).await;
        format!("Waited form {} seconds", seconds)
    }

    #[post("/", data = "<user>")]
    pub fn delay_post(user: Json<User>) -> Json<User> {
        Json(User {
            name: user.name.clone(),
            email: user.email.clone()
        })
    }

}