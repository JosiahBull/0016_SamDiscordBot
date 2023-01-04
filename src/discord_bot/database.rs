pub type DatabaseResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub mod shopping {
    use crate::state::AppState;
    use chrono::Local;
    use sea_orm::ActiveValue;
    use sea_orm::ColumnTrait;
    use sea_orm::EntityTrait;
    use sea_orm::IntoActiveModel;
    use sea_orm::ModelTrait;
    use sea_orm::QueryFilter;
    use sea_orm::QueryOrder;
    use sea_orm::QuerySelect;
    use serenity::async_trait;

    use super::DatabaseResult;

    pub use entity::list::ActiveModel as ShoppingListActiveModel;
    pub use entity::list::Entity as ShoppingListEntity;
    pub use entity::list::Model as ShoppingListModel;

    pub use entity::list_item::ActiveModel as ShoppingListItemActiveModel;
    pub use entity::list_item::Entity as ShoppingListItemEntity;
    pub use entity::list_item::Model as ShoppingListItemModel;

    use sea_orm::ActiveModelTrait;

    pub struct NewShoppingListItem<'a> {
        pub item: &'a str,
        pub store: Option<&'a str>,
        pub notes: Option<&'a str>,
        pub quantity: i64,
        pub personal: bool,
    }

    #[async_trait]
    pub trait SerenityShoppingDatabase {
        async fn get_shopping_list(
            &self,
        ) -> DatabaseResult<Option<(ShoppingListModel, Vec<ShoppingListItemModel>)>>;

        async fn add_shopping_list_item(
            &self,

            user: u64,
            message_id: u64,
            channel_id: u64,
            guild_id: Option<u64>,

            item: NewShoppingListItem<'_>,
        ) -> DatabaseResult<()>;

        async fn get_shopping_list_item_by_id(
            &self,
            id: u64,
        ) -> DatabaseResult<Option<ShoppingListItemModel>>;

        async fn get_shopping_list_item_by_message_id(
            &self,
            message_id: u64,
        ) -> DatabaseResult<Option<ShoppingListItemModel>>;

        async fn set_shopping_list_item_bought(
            &self,
            user: u64,
            message_id: u64,
            setting: bool,
        ) -> DatabaseResult<()>;

        async fn get_recent_shopping_list_items_by_user(
            &self,
            user: u64,
            count: u64,
        ) -> DatabaseResult<Vec<ShoppingListItemModel>>;

        async fn get_recent_shopping_list_items(
            &self,
            count: u64,
        ) -> DatabaseResult<Vec<ShoppingListItemModel>>;
    }

    #[async_trait]
    impl SerenityShoppingDatabase for AppState {
        async fn get_shopping_list(
            &self,
        ) -> DatabaseResult<Option<(ShoppingListModel, Vec<ShoppingListItemModel>)>> {
            // try to load the most recent shopping list from the database
            // XXX: should work for multiple guilds at some point
            let shopping_list: Option<ShoppingListModel> = ShoppingListEntity::find()
                .order_by_desc(<ShoppingListEntity as EntityTrait>::Column::CreatedAt)
                .one(&*self.database)
                .await?;

            if shopping_list.is_none() {
                return Ok(None);
            }
            let shopping_list = shopping_list.unwrap();

            // load the items for the shopping list
            let items = shopping_list
                .find_related(ShoppingListItemEntity)
                .all(&*self.database)
                .await?;

            Ok(Some((shopping_list, items)))
        }

        async fn add_shopping_list_item(
            &self,

            user: u64,
            message_id: u64,
            channel_id: u64,
            guild_id: Option<u64>,

            item: NewShoppingListItem<'_>,
        ) -> DatabaseResult<()> {
            // try to load the most recent shopping list from the database, and add the item to it
            // if no shopping list exists, create a new one

            let shopping_list: Option<(ShoppingListModel, Vec<ShoppingListItemModel>)> =
                self.get_shopping_list().await?;

            let shopping_list = match shopping_list {
                Some(s) => s.0,
                None => {
                    let shopping_list = ShoppingListActiveModel {
                        id: ActiveValue::NotSet,
                        name: ActiveValue::Set(item.item.to_string()),
                        created_by: ActiveValue::Set(user as i64),
                        created_at: ActiveValue::Set(Local::now().naive_local()),
                        creation_message_id: ActiveValue::Set(message_id as i64),
                        creation_message_channel_id: ActiveValue::Set(channel_id as i64),
                        creation_message_guild_id: ActiveValue::Set(guild_id.map(|g| g as i64)),
                    };

                    shopping_list.insert(&*self.database).await?;

                    ShoppingListEntity::find()
                        .order_by_desc(<ShoppingListEntity as EntityTrait>::Column::CreatedAt)
                        .one(&*self.database)
                        .await?
                        .unwrap()
                }
            };

            let item = ShoppingListItemActiveModel {
                id: ActiveValue::NotSet,
                list_id: ActiveValue::Set(shopping_list.id),
                message_id: ActiveValue::Set(message_id as i64),
                user_id: ActiveValue::Set(user as i64),

                created_at: ActiveValue::Set(Local::now().naive_local()),
                bought: ActiveValue::Set(false),

                item: ActiveValue::Set(item.item.to_string()),
                quantity: ActiveValue::Set(item.quantity),
                personal: ActiveValue::Set(item.personal),
                store: ActiveValue::Set(item.store.map(|s| s.to_string())),
                notes: ActiveValue::Set(item.notes.map(|n| n.to_string())),
            };
            item.insert(&*self.database).await?;

            Ok(())
        }

        async fn get_shopping_list_item_by_id(
            &self,
            id: u64,
        ) -> DatabaseResult<Option<ShoppingListItemModel>> {
            let item = ShoppingListItemEntity::find()
                .filter(<ShoppingListItemEntity as EntityTrait>::Column::Id.eq(id as i64))
                .one(&*self.database)
                .await?;

            Ok(item)
        }

        async fn get_shopping_list_item_by_message_id(
            &self,
            message_id: u64,
        ) -> DatabaseResult<Option<ShoppingListItemModel>> {
            let item = ShoppingListItemEntity::find()
                .filter(
                    <ShoppingListItemEntity as EntityTrait>::Column::MessageId
                        .eq(message_id as i64),
                )
                .one(&*self.database)
                .await?;

            Ok(item)
        }

        async fn set_shopping_list_item_bought(
            &self,
            user: u64,
            message_id: u64,
            setting: bool,
        ) -> DatabaseResult<()> {
            let shopping_list_item = ShoppingListItemEntity::find()
                .filter(
                    <ShoppingListItemEntity as EntityTrait>::Column::MessageId
                        .eq(message_id as i64),
                )
                .filter(<ShoppingListItemEntity as EntityTrait>::Column::UserId.eq(user as i64))
                .one(&*self.database)
                .await?;

            if let Some(shopping_list_item) = shopping_list_item {
                let mut shopping_list_item = shopping_list_item.into_active_model();
                shopping_list_item.bought = ActiveValue::Set(setting);
                shopping_list_item.update(&*self.database).await?;
            }

            Ok(())
        }

        async fn get_recent_shopping_list_items_by_user(
            &self,
            user: u64,
            count: u64,
        ) -> DatabaseResult<Vec<ShoppingListItemModel>> {
            let shopping_list: Vec<ShoppingListItemModel> = ShoppingListItemEntity::find()
                .filter(<ShoppingListItemEntity as EntityTrait>::Column::UserId.eq(user as i64))
                .order_by_desc(<ShoppingListItemEntity as EntityTrait>::Column::CreatedAt)
                .limit(count)
                .all(&*self.database)
                .await?;

            Ok(shopping_list)
        }

        async fn get_recent_shopping_list_items(
            &self,
            count: u64,
        ) -> DatabaseResult<Vec<ShoppingListItemModel>> {
            let shopping_list: Vec<ShoppingListItemModel> = ShoppingListItemEntity::find()
                .order_by_desc(<ShoppingListItemEntity as EntityTrait>::Column::CreatedAt)
                .limit(count)
                .all(&*self.database)
                .await?;

            Ok(shopping_list)
        }
    }
}
