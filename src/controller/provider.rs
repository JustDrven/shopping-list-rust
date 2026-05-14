use axum::{Router, routing::{delete, get, post, put}};
use crate::controller::item::{
    complete_shopping, 
    delete_specific_item, 
    
    get_item, 
    get_items, 
    
    push_new_item
};


pub fn initialize_controllers() -> Router {
    return Router::new()
        .route("/items/complete", put(complete_shopping))
        .route("/items", post(push_new_item))
        .route("/items", delete(delete_specific_item))

        .route("/items", get(get_items))
        .route("/items/{id}", get(get_item));
}
