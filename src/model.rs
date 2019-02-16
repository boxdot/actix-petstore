use serde::{Deserialize, Serialize};

/// A pet for sale in the pet store
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pet {
    id: Option<i64>,
    category: Option<Category>,
    name: String,
    photo_urls: Vec<String>,
    tags: Option<Vec<Tag>>,
    status: Option<Status>,
}

/// Pet status in the store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Available,
    Pending,
    Sold,
}

/// A category for a pet
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Category {
    id: Option<i64>,
    name: Option<String>,
}

/// A tag for a pet
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tag {
    id: Option<i64>,
    name: Option<String>,
}

/// An uploaded response
///
/// Describes the result of uploading an image resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    code: Option<i32>,
    _type: Option<String>,
    message: Option<String>,
}

/// Pet Order
///
/// An order for a pets from the pet store
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    id: Option<i64>,
    pet_id: Option<i64>,
    quantity: Option<i32>,
    ship_date: Option<String>,
    status: Option<OrderStatus>,
    #[serde(default)]
    complete: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum OrderStatus {
    Placed,
    Approved,
    Delivered,
}

/// a User
///
/// A User who is purchasing from the pet store
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: Option<i64>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    phone: Option<String>,
    /// User Status
    user_status: Option<i32>,
}

/// List of user object
pub type UserArray = Vec<User>;
