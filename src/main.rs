mod model;

use model::*;

use actix_web::{http, server, App, Form, HttpRequest, HttpResponse, Json, Path, Query, Result};
use serde::Deserialize;

use std::collections::HashMap;
use std::env;

/// Add a new pet to the store
fn add_pet(_pet: Json<Pet>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Update an existing pet
fn update_pet(_pet: Json<Pet>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Debug, Deserialize)]
struct FindPetsByStatusQuery {
    status: Vec<Status>,
}

/// Finds Pets by status
///
/// Multiple status values can be provided with comma separated strings
fn find_pets_by_status(_query: Query<FindPetsByStatusQuery>) -> Result<Json<Vec<Pet>>> {
    unimplemented!();
}

/// Finds Pets by tags
///
/// Multiple tags can be provided with comma separated strings. Use tag1, tag2, tag3 for testing.
fn find_pets_by_tags(_query: Query<FindPetsByStatusQuery>) -> Result<Json<Vec<Pet>>> {
    unimplemented!();
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PetIdPath {
    pet_id: i64,
}

/// Find pet by ID
///
/// Returns a single pet
fn get_pet_by_id(_path: Path<PetIdPath>) -> Result<Json<Pet>> {
    unimplemented!();
}

#[derive(Debug, Deserialize)]
struct UpdatePetByIdForm {
    /// Updated name of the pet
    name: Option<String>,
    /// Updated status of the pet
    status: Option<Status>,
}

/// Updates a pet in the store with form data
fn update_pet_with_form(
    (_path, _form): (Path<PetIdPath>, Form<UpdatePetByIdForm>),
) -> HttpResponse {
    unimplemented!();
}

/// Deletes a pet
fn delete_pet(_path: Path<PetIdPath>) -> HttpResponse {
    // TODO: extract header
    unimplemented!();
}

/// Uploads an image
fn upload_file(_path: Path<PetIdPath>) -> Result<Json<ApiResponse>> {
    // TODO: Describe multiform data.
    unimplemented!();
}

/// Returns pet inventories by status
///
/// Returns a map of status codes to quantities
fn get_inventory(_req: HttpRequest) -> Result<Json<HashMap<String, i32>>> {
    unimplemented!();
}

/// Place an order for a pet
fn place_order(_order: Json<Order>) -> Result<Json<Order>> {
    unimplemented!();
}

/// Find purchase order by ID
///
/// For valid response try integer IDs with value <= 5 or > 10. Other values
/// will generated exceptions
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OrderIdPath {
    // TODO: Add validation 1 <= x <= 5
    order_id: i64,
}

/// Find purchase order by ID
fn get_order_by_id(_path: Path<OrderIdPath>) -> Result<Json<Order>> {
    unimplemented!();
}

/// Delete purchase order by ID
fn delete_order(_path: Path<OrderIdPath>) -> HttpResponse {
    unimplemented!();
}

/// Create user
///
/// This can only be done by the logged in user.
fn create_user(_user: Json<User>) -> HttpResponse {
    unimplemented!();
}

/// Creates list of users with given input array
fn create_users_with_array_input(_user: Json<UserArray>) -> HttpResponse {
    unimplemented!();
}

/// Creates list of users with given input array
fn create_users_with_list_input(_user: Json<UserArray>) -> HttpResponse {
    unimplemented!();
}

#[derive(Debug, Deserialize)]
struct LoginUserQuery {
    /// The user name for login
    username: String,
    /// The password for login in clear text
    password: String,
}

/// Logs user into the system
fn login_user(_query: Query<LoginUserQuery>) -> Result<String> {
    // TODO: can we specify headers in Response
    unimplemented!();
}

/// Logs out current logged in user session
fn logout_user(_query: Query<LoginUserQuery>) -> HttpResponse {
    unimplemented!();
}

#[derive(Debug, Deserialize)]
struct UsernamePath {
    /// The name that needs to be fetched. Use user1 for testing.
    username: String,
}

/// Get user by user name
fn get_user_by_name(_path: Path<UsernamePath>) -> Json<User> {
    unimplemented!();
}

/// Update user
///
/// This can only be done by the logged in user.
fn update_user(_path: Path<UsernamePath>) -> Json<User> {
    unimplemented!();
}

/// Delete user
///
/// This can only be done by the logged in user.
fn delete_user(_path: Path<UsernamePath>) -> HttpResponse {
    unimplemented!();
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    server::new(|| {
        App::new()
            .resource("/pet", |r| {
                r.post().with(add_pet);
                r.put().with(update_pet);
            })
            .resource("/pet/findByStatus", |r| r.get().with(find_pets_by_status))
            .resource("/pet/findByTags", |r| r.get().with(find_pets_by_tags))
            .resource("/pet/{petId}", |r| {
                r.get().with(get_pet_by_id);
                r.post().with(update_pet_with_form);
                r.method(http::Method::DELETE).with(delete_pet);
            })
            .resource("/pet/{petId}/uploadImage", |r| r.post().with(upload_file))
            .resource("/store/inventory", |r| r.get().with(get_inventory))
            .resource("/store/order", |r| r.post().with(place_order))
            .resource("/store/order/{orderId}", |r| {
                r.get().with(get_order_by_id);
                r.method(http::Method::DELETE).with(delete_order);
            })
            .resource("/user", |r| r.post().with(create_user))
            .resource("/user/createWithArray", |r| {
                r.post().with(create_users_with_array_input)
            })
            .resource("/user/createWithArray", |r| {
                r.post().with(create_users_with_array_input)
            })
            .resource("/user/createWithList", |r| {
                r.post().with(create_users_with_list_input)
            })
            .resource("/user/login", |r| r.get().with(login_user))
            .resource("/user/logout", |r| r.get().with(logout_user))
            .resource("/user/{username}", |r| {
                r.get().with(get_user_by_name);
                r.method(http::Method::PUT).with(update_user);
                r.method(http::Method::DELETE).with(delete_user);
            })
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .shutdown_timeout(0)
    .run();
}
