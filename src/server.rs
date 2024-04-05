use dioxus::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


use crate::{User, UserType};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ServerUser {
    pub username: String,
    pub password: String,
    pub user_type: UserType,
    login_attempts: Vec<DateTime<Utc>>
}

impl ServerUser {
    pub fn from_user(user: User) -> Self {
        Self {
            username: user.username,
            password: user.password,
            user_type: user.user_type,
            login_attempts: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerResponse {
    LoginSuccess(User),
    LoginFailure(String),
    SignupSuccess,
    SignupFailure(String),
}

#[cfg(feature = "server")]
fn validate_user(user: &User) -> Result<(), String> {
    let mut errors: Vec<String> = vec![];

    // Username checks
    // At least 4 characters long
    if user.username.len() < 4 {
        errors.push("Username must be at least 3 characters long".to_string());
    }
    // All lowercase
    if user.username.to_ascii_lowercase() != user.username {
        errors.push("Username must be lowercase".to_string());
    }
    // A single underscore
    if !user.username.contains('_') || user.username.chars().filter(|&c| c == '_').count() > 1 {
        errors.push("Username must contain a single underscore".to_string());
    }

    // Password checks
    // At least 8 characters long
    if user.password.len() < 8 {
        errors.push("Password must be at least 8 characters long".to_string());
    }
    // At least 1 capital
    if user.password.chars().filter(|&c| c.is_uppercase()).count() == 0 {
        errors.push("Password must contain at least 1 capital letter".to_string());
    }
    // At least 1 non-'.' special character
    if user
        .password
        .chars()
        .filter(|&c| c.is_ascii_punctuation())
        .count()
        == 0
        || user.password.contains('.')
    {
        errors.push("Password must contain at least 1 non-'.' special character".to_string());
    }

    if !errors.is_empty() {
        return Err(errors.join(", "));
    }

    Ok(())
}

#[server]
pub async fn signup_user(user: User) -> Result<ServerResponse, ServerFnError> {
    // Import the Surreal database
    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::Surreal;

    println!("Server received sign-up request for {user:?}");

    match validate_user(&user) {
        Ok(()) => (),
        Err(e) => return Ok(ServerResponse::SignupFailure(e)),
    }

    let user = ServerUser::from_user(user);

    // Create database connection
    let db = match Surreal::new::<Ws>("localhost:8000").await {
        Ok(db) => db,
        Err(e) => return Ok(ServerResponse::SignupFailure(e.to_string())),
    };

    // Select a specific namespace / database
    db.use_ns("movies").use_db("users").await.unwrap();

    // Check if this user exists
    let result: Option<ServerUser> = db.select(("user", user.username.clone())).await.unwrap();
    dbg!(&result);
    if result.is_some() {
        return Ok(ServerResponse::SignupFailure(
            "Username already exists".to_string(),
        ));
    }

    // Create a new user
    let user: Option<ServerUser> = db
        .create(("user", user.username.clone()))
        .content(user)
        .await
        .unwrap();

    dbg!(user);

    Ok(ServerResponse::SignupSuccess)
}

#[server]
pub async fn login_user(user: User) -> Result<ServerResponse, ServerFnError> {
    // Import the Surreal database
    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::Surreal;

    println!("server received login request for {user:?}");

    match validate_user(&user) {
        Ok(()) => (),
        Err(e) => return Ok(ServerResponse::LoginFailure(e)),
    }

    let server_user = ServerUser::from_user(user.clone());

    // Create database connection
    let db = match Surreal::new::<Ws>("localhost:8000").await {
        Ok(db) => db,
        Err(e) => return Ok(ServerResponse::LoginFailure(e.to_string())),
    };

    // Select a specific namespace / database
    db.use_ns("movies").use_db("users").await.unwrap();

    // Validate password
    let result: Option<ServerUser> = db.select(("user", server_user.username.clone())).await.unwrap();
    dbg!(&result);
    if let Some(mut existing_user) = result {
        if existing_user.password != server_user.password {
            let incorrects: usize = existing_user.login_attempts.iter().filter(|&&t| 
                t >= Utc::now() - chrono::Duration::days(5)
            ).count();

            println!("{server_user:#?}");

            // User is bad - delete account
            if incorrects > 4 {
                let _: Option<ServerUser> = db.delete(("user", server_user.username.clone())).await.unwrap();
                return Ok(ServerResponse::LoginFailure(
                    "Incorrect Password! Too many attempts - account deleted!".to_string(),
                ));
            } 
            // Add a login attempt
            existing_user.login_attempts.push(Utc::now());
            let _: Option<ServerUser> = db.update(("user", server_user.username.clone())).content(existing_user).await.unwrap();
            // Fail the login
            return Ok(ServerResponse::LoginFailure(
                format!("Incorrect Password! {} Attempts left.", 5 - incorrects),
            ));
        }
        
        return Ok(ServerResponse::LoginSuccess(user));
    }
    // No user in database
    Ok(ServerResponse::LoginFailure("User not found".to_string()))
}
