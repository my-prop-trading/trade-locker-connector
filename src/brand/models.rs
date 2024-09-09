use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckEmailRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckEmailResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetUserPasswordRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub password: String,
}
