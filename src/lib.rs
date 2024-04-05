use serde::{Deserialize, Serialize};

pub mod pages;
pub mod routes;
pub mod server;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct User {
    pub username: String,
    pub password: String,
    pub user_type: UserType,
}

/// User types defining what priviledges are available to a user.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub enum UserType {
    /// Has full access to all features; can modify `UserType`s.
    Admin,
    /// Can see statistics and user feedback. Provides feedback to
    /// `ContentEditor`s.
    MarketingManager,
    /// Can see `MarketingManager` feedback and add/delete movies.
    ContentEditor,
    /// Can view movies and leave reviews.
    #[default]
    Viewer,
}


/// User types defining what priviledges are available to a user.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Movie {
    pub title: String,
    pub description: String,
    pub link: String,
    pub rating: f32,
    pub reviews: Vec<u8>,
}