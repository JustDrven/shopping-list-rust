pub mod query {
    use std::sync::LazyLock;
    use axum::extract::Path;
    use moka::future::Cache;

    use crate::application::dto::item::ItemDto;
    use crate::application;
    use crate::application::repository::item::function;

    static CACHE: LazyLock<Cache<i32, ItemDto>> = LazyLock::new(|| {
        application::util::cache::create_item()
    });
    static LIST_CACHE: LazyLock<Cache<u8, Vec<ItemDto>>> = LazyLock::new(|| {
        application::util::cache::create_items()
    });



    pub(crate) async fn get_item(id: Path<i32>) -> ItemDto {
        let mut final_id: i32 = id.0;
        match CACHE.get(&final_id).await {
            Some(item) => item,
            None => {
                let new_dto = function::load_item_from_database(&mut final_id).await;
                CACHE.insert(final_id, new_dto.clone()).await;
                new_dto
            }
        }

    }

    pub(crate) async fn get_items() -> Vec<ItemDto> {
        const DEFAULT_ID: u8 = 0;

        match LIST_CACHE.get(&DEFAULT_ID).await {
            Some(item) => item,
            None => {
                let new_dto: Vec<ItemDto> = function::load_items_from_database().await;
                LIST_CACHE.insert(DEFAULT_ID, new_dto.to_vec()).await;
                new_dto.to_vec()
            }
        }
    }


}

pub mod mutation {
    use axum::Json;
    use crate::application::payload::request;
    use crate::application::payload::response;

    use crate::application::repository::item::function;

    pub(crate) async fn complete(Json(data): Json<request::CompleteRequest>) -> response::OkResponse {
        function::complete_async(&data.into()).await;

        response::ok_response(true, String::from("complete"))
    }

    pub(crate) async fn delete(Json(data): Json<request::DeleteRequest>) -> response::OkResponse {
        function::delete_async(&data.into()).await;
        response::ok_response(true, String::from("delete"))
    }

    pub(crate) async fn create(Json(data): Json<request::CreateRequest>) -> response::OkResponse {
        function::create_async(&data.into()).await;
        response::ok_response(true, String::from("create"))
    }

}

mod function {
    use sea_orm::{
        ActiveModelTrait, EntityTrait, Set
    };

    use crate::application::orm::database;
    use crate::application::dto::item::ItemDto;

    use crate::application::models::item;
    use crate::application::models::item::Entity as ItemEntity;

    use crate::application::payload::request;

    pub async fn create_async(data: &request::CreateRequest) {
        let name: &String = &data.name;
        let pool = database::get_pool();

        match pool {
            Some(db) => {
                let new_item = item::ActiveModel {
                    name: Set(name.to_owned()),
                    complete: Set(false),
                    ..Default::default()
                };

                new_item.insert(&db).await.unwrap();

            }
            None => {}
        }


    }

    pub async fn delete_async(data: &request::DeleteRequest) {
        let id: i32 = data.id;

        let pool = database::get_pool();
        match pool {
            Some(pool) => {
                let item = ItemEntity::find_by_id(id)
                    .one(&pool)
                    .await.unwrap();

                let active: item::ActiveModel = item.unwrap().into();
                active.delete(&pool).await.unwrap();
            }
            None => {}
        }
    }


    pub async fn complete_async(data: &request::CompleteRequest) {
        let id: i32 = data.id;

        let pool = database::get_pool();
        match pool {
            Some(pool) => {
                let item = ItemEntity::find_by_id(id)
                    .one(&pool)
                    .await.unwrap();

                let mut active: item::ActiveModel = item.unwrap().into();
                active.complete = Set(true);
                active.update(&pool).await.unwrap();
            }
            None => {}
        }

    }

    pub async fn load_item_from_database(id: &mut i32) -> ItemDto {
        let pool = database::get_pool();

        match pool {
            Some(pool) => {

                let item_model = ItemEntity::find_by_id(*id)
                    .one(&pool)
                    .await.unwrap().unwrap();

                ItemDto {
                    id: item_model.id,
                    name: item_model.name,
                    complete: item_model.complete,
                }

            }
            _ =>  ItemDto {
                id: -1,
                name: "".to_string(),
                complete: false,
            }
        }
    }

    pub async fn load_items_from_database() -> Vec<ItemDto> {
        let pool = database::get_pool();

        match pool {
            Some(pool) => {

                let list = ItemEntity::find()
                    .all(&pool)
                    .await.unwrap();

                let mut to_return: Vec<ItemDto> = Vec::new();
                for x in list {
                    let dto = ItemDto {
                        id: x.id,
                        name: x.name,
                        complete: x.complete
                    };

                    to_return.push(dto);
                }

                to_return
            }
            _ => vec![]
        }


    }

}

