use entity::list::Entity as ShoppingListEntity;
use entity::list::Model as ShoppingList;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::ActiveModelTrait;
use serenity::async_trait;

use crate::AppState;

pub type DatabaseResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub mod shopping {
    use crate::state::AppState;
    use sea_orm::ActiveValue;
    use sea_orm::EntityTrait;
    use sea_orm::QueryOrder;
    use serenity::async_trait;

    use super::DatabaseResult;

    use entity::list::ActiveModel as ShoppingListActiveModel;
    use entity::list::Entity as ShoppingListEntity;
    use entity::list::Model as ShoppingListModel;
    use sea_orm::prelude::DateTimeUtc;
    use sea_orm::ActiveModelTrait;

    pub struct ShoppingListItem<'a> {
        pub item: &'a str,
        pub personal: bool,
        pub quantity: Option<i64>,
        pub store: Option<&'a str>,
        pub notes: Option<&'a str>,
    }

    #[async_trait]
    pub trait SerenityShoppingDatabase {
        async fn get_shopping_list(&self) -> DatabaseResult<Option<ShoppingListModel>>;

        async fn add_shopping_list_item(
            &self,

            user: u64,
            message_id: u64,

            item: ShoppingListItem<'_>,
        ) -> DatabaseResult<()>;

        async fn remove_shopping_list_item(&self, user: u64, message_id: u64)
            -> DatabaseResult<()>;
    }

    #[async_trait]
    impl SerenityShoppingDatabase for AppState {
        async fn get_shopping_list(&self) -> DatabaseResult<Option<ShoppingListModel>> {
            // try to load the most recent shopping list from the database
            // XXX: should work for multiple guilds at some point
            let shopping_list: Option<ShoppingListModel> = ShoppingListEntity::find()
                .order_by_desc(<ShoppingListEntity as EntityTrait>::Column::CreatedAt)
                .one(&*self.database)
                .await?;

            match shopping_list {
                Some(ref s) if !s.bought => Ok(shopping_list),
                _ => Ok(None),
            }
        }

        async fn add_shopping_list_item(
            &self,

            user: u64,
            message_id: u64,

            item: ShoppingListItem<'_>,
        ) -> DatabaseResult<()> {
            // let model: ShoppingListActiveModel = ShoppingListActiveModel {
            //     id: ActiveValue::NotSet,
            //     name: ActiveValue::Set(String::from("")),
            //     created_by: ,
            //     created_at: sea_orm::prelude::DateTime::from_timestamp_millis(0).unwrap(),
            //     original_list_message_id: 0,
            //     item_message_ids: vec![],
            //     items: vec![],
            //     item_amounts: vec![],
            // };
            // model.insert(self.database).await?
            todo!()
        }

        async fn remove_shopping_list_item(
            &self,

            user: u64,
            message_id: u64,
        ) -> DatabaseResult<()> {
            todo!()
        }
    }
}

// #[async_trait]
// pub trait SerenityDatabase {
//     async fn add_shopping_list_item(
//         &self,
//         // item: &str,
//         // personal: bool,
//         // quantity: Option<u32>,
//         // store: Option<&str>,
//         // notes: Option<&str>,
//     ) -> DatabaseResult<()>;

//     async fn get_shopping_list(&self) -> DatabaseResult<ShoppingList> {
//         Ok(ShoppingList {
//             id: 0,
//             name: String::from(""),
//             created_by: 0,
//             created_at: sea_orm::prelude::DateTime::from_timestamp_millis(0).unwrap(),
//             original_list_message_id: 0,
//             item_message_ids: vec![],
//             items: vec![],
//             item_amounts: vec![],
//         })
//     }

//     async fn get_shopping_list_with_id(&self, shopping_list_id: u32) -> DatabaseResult<()> {
//         todo!()
//     }

//     async fn remove_shopping_list_item(
//         &self,
//         shopping_list_id: u32,
//         item_id: u32,
//     ) -> DatabaseResult<()> {
//         todo!()
//     }
// }

// #[async_trait]
// impl SerenityDatabase for AppState {
//     async fn add_shopping_list_item(
//         &self,
//         item: &str,
//         personal: bool,
//         quantity: Option<u32>,
//         store: Option<&str>,
//         notes: Option<&str>,
//     ) -> DatabaseResult<()> {
//         let friend: entity::list::ActiveModel = ShoppingList {
//             id: 0,
//             name: String::from(""),
//             created_by: 0,
//             created_at: sea_orm::prelude::DateTime::from_timestamp_millis(0).unwrap(),
//             original_list_message_id: 0,
//             item_message_ids: vec![],
//             items: vec![],
//             item_amounts: vec![],
//         }
//         .into();

//         friend.insert(&*self.database).await?;

//         Ok(())
//     }

//     // async fn get_shopping_list(&self) -> DatabaseResult<ShoppingList> {
//     //     todo!()
//     // }

//     async fn get_shopping_list_with_id(&self, shopping_list_id: u32) -> DatabaseResult<()> {
//         todo!()
//     }

//     async fn remove_shopping_list_item(
//         &self,
//         shopping_list_id: u32,
//         item_id: u32,
//     ) -> DatabaseResult<()> {
//         todo!()
//     }
// }
