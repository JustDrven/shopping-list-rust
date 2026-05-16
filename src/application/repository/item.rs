pub mod query {
    use std::sync::LazyLock;
    use axum::extract::Path;
    use moka::future::Cache;

    use crate::application::dto::item::ItemDto;
    use crate::application;
    use crate::application::repository::item::function;

    static CACHE: LazyLock<Cache<String, ItemDto>> = LazyLock::new(|| {
        application::util::cache::create_item()
    });
    static LIST_CACHE: LazyLock<Cache<u8, Vec<ItemDto>>> = LazyLock::new(|| {
        application::util::cache::create_items()
    });



    pub(crate) async fn get_item(id: Path<String>) -> ItemDto {
        let final_id: &str = id.as_str();
        match CACHE.get(final_id).await {
            Some(item) => item,
            None => {
                let new_dto = function::load_item_from_database(id.as_ref()).await;
                CACHE.insert(id.as_str().to_string(), new_dto.clone()).await;
                new_dto
            }
        }

    }

    pub(crate) async fn get_items() -> Vec<ItemDto> {
        const DEFAULT_ID: u8 = 0;

        match LIST_CACHE.get(&DEFAULT_ID).await {
            Some(item) => item,
            None => {
                let new_dto: Vec<ItemDto> = function::load_items_from_database();
                LIST_CACHE.insert(DEFAULT_ID, new_dto.to_vec()).await;
                new_dto.to_vec()
            }
        }
    }


}

pub mod mutation {
    use std::sync::LazyLock;
    use axum::Json;
    use crate::application::payload::request;
    use crate::application::payload::response;

    use threadpool::ThreadPool;
    use crate::application::repository::item::function;

    const THREAD_POOL: LazyLock<ThreadPool> = LazyLock::new(|| ThreadPool::new(10));

    pub(crate) fn complete(Json(data): Json<request::CompleteRequest>) -> response::OkResponse {
        THREAD_POOL.execute(|| {
            function::complete_async(&data.into());
        });

        response::ok_response(true, String::from("complete"))
    }

    pub(crate) fn delete(Json(data): Json<request::DeleteRequest>) -> response::OkResponse {
        THREAD_POOL.execute(|| {
            function::delete_async(&data.into());
        });

        response::ok_response(true, String::from("delete"))
    }

    pub(crate) fn create(Json(data): Json<request::CreateRequest>) -> response::OkResponse {
        THREAD_POOL.execute(|| {
            function::create_async(&data.into());
        });


        response::ok_response(true, String::from("create"))
    }



}

mod function {
    use crate::application::dto::item::ItemDto;
    use crate::application::payload::request;

    pub fn create_async(data: &request::CreateRequest) {
        let name: &String = &data.0;

    }

    pub fn delete_async(data: &request::DeleteRequest) {
        let id: u32 = data.0;
    }

    pub fn complete_async(data: &request::CompleteRequest) {
        let id: u32 = data.0;

    }

    pub async fn load_item_from_database(id: &str) -> ItemDto {
        ItemDto {
            id: 3434,
            name: id.into(),
        }
    }

    pub fn load_items_from_database() -> Vec<ItemDto> {
        let items: Vec<ItemDto> = Vec::new();

        items
    }

}

