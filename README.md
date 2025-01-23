# sea-orm-crud-macros

## Usage

- add `sea-orm-crud-macros = { git = "https://github.com/wdcodecn/sea-orm-crud-macros"}` to `Cargo.toml`

```toml

#file: Cargo.toml 

[dependencies]
sea-orm-crud-macros = { git = "https://github.com/wdcodecn/sea-orm-crud-macros"}

```

### Generate Seaorm Entity

> see `--model-extra-derives 'sea_orm_crud_macros::SeaOrmCrud'`

```sh
sea-orm-cli generate entity --with-serde both --model-extra-derives 'sea_orm_crud_macros::SeaOrmCrud' --output-dir src/entities --database-url postgres://postgres:1234qwer@localhost:5432/loco
sed -i 's/ Decimal,/ BigDecimal,/g' src/entities/*
```

### Example 

```rust

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, sea_orm_crud_macros :: SeaOrmCrud, )]
#[sea_orm(table_name = "users")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32, 
    pub password: String, 
    pub name: String, 
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


```

### Expand Code Here
```rust

#[derive(Debug)]
pub struct PageOption {
    page: u64,
    page_size: u64,
}
#[doc = r" 默认分页"]
impl Default for PageOption {
    fn default() -> Self {
        PageOption {
            page: 1,
            page_size: 10,
        }
    }
}
#[derive(Debug)]
pub struct SortOption {
    name: Column,
    desc: bool,
}
#[doc = r" 默认排序 CreatedAt 倒序"]
impl Default for SortOption {
    fn default() -> Self {
        SortOption {
            name: Column::CreatedAt,
            desc: true,
        }
    }
}
#[derive(Debug, Default)]
pub struct OptionModel {
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub id: Option<i32>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub sort_option: Option<SortOption>,
    pub page_option: Option<PageOption>,
}
use sea_orm::ActiveValue::Set;
use sea_orm::ItemsAndPagesNumber;
use sea_orm::QueryOrder;
pub struct Service {}
impl Service {
    pub async fn insert_one(db: &DbConn, x: ActiveModel) -> Result<Model, DbErr> {
        x.insert(db).await
    }
    pub async fn get_one_by_id(db: &DbConn, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }
    pub async fn find_by_option_model(
        db: &DbConn,
        x: OptionModel,
    ) -> Result<(Vec<Model>, ItemsAndPagesNumber), DbErr> {
        let mut query = Entity::find();
        if let Some(n) = x.created_at {
            query = query.filter(Column::CreatedAt.eq(n));
        }
        if let Some(n) = x.updated_at {
            query = query.filter(Column::UpdatedAt.eq(n));
        }
        if let Some(n) = x.id {
            query = query.filter(Column::Id.eq(n));
        }
        if let Some(n) = x.password {
            query = query.filter(Column::Password.eq(n));
        }
        if let Some(n) = x.name {
            query = query.filter(Column::Name.eq(n));
        }
        let sort_option = x.sort_option.unwrap_or_default();
        query = if sort_option.desc {
            query.order_by_desc(sort_option.name)
        } else {
            query.order_by_asc(sort_option.name)
        };
        let page_option = x.page_option.unwrap_or_default();
        let paginator = query.paginate(db, page_option.page_size);
        let page: ItemsAndPagesNumber = paginator.num_items_and_pages().await?;
        let data = paginator.fetch_page(page_option.page - 1).await?;
        Ok((data, page))
    }
    pub async fn get_by_created_at(
        db: &DbConn,
        created_at: DateTimeWithTimeZone,
        sort_option: Option<SortOption>,
        page_option: Option<PageOption>,
    ) -> Result<(Vec<Model>, ItemsAndPagesNumber), DbErr> {
        Self::find_by_option_model(db, OptionModel {
            created_at: Some(created_at),
            sort_option,
            page_option,
            ..Default::default()
        })
            .await
    }
    pub async fn update_created_at(
        db: &DbConn,
        id: i32,
        created_at: DateTimeWithTimeZone,
    ) -> Result<Model, DbErr> {
        let x = ActiveModel {
            id: Set(id),
            created_at: Set(created_at),
            ..Default::default()
        };
        x.update(db).await
    }
    pub async fn get_by_updated_at(
        db: &DbConn,
        updated_at: DateTimeWithTimeZone,
        sort_option: Option<SortOption>,
        page_option: Option<PageOption>,
    ) -> Result<(Vec<Model>, ItemsAndPagesNumber), DbErr> {
        Self::find_by_option_model(db, OptionModel {
            updated_at: Some(updated_at),
            sort_option,
            page_option,
            ..Default::default()
        })
            .await
    }
    pub async fn update_updated_at(
        db: &DbConn,
        id: i32,
        updated_at: DateTimeWithTimeZone,
    ) -> Result<Model, DbErr> {
        let x = ActiveModel {
            id: Set(id),
            updated_at: Set(updated_at),
            ..Default::default()
        };
        x.update(db).await
    }
    pub async fn get_by_password(
        db: &DbConn,
        password: String,
        sort_option: Option<SortOption>,
        page_option: Option<PageOption>,
    ) -> Result<(Vec<Model>, ItemsAndPagesNumber), DbErr> {
        Self::find_by_option_model(db, OptionModel {
            password: Some(password),
            sort_option,
            page_option,
            ..Default::default()
        })
            .await
    }
    pub async fn update_password(db: &DbConn, id: i32, password: String) -> Result<Model, DbErr> {
        let x = ActiveModel {
            id: Set(id),
            password: Set(password),
            ..Default::default()
        };
        x.update(db).await
    }
    pub async fn get_by_name(
        db: &DbConn,
        name: String,
        sort_option: Option<SortOption>,
        page_option: Option<PageOption>,
    ) -> Result<(Vec<Model>, ItemsAndPagesNumber), DbErr> {
        Self::find_by_option_model(db, OptionModel {
            name: Some(name),
            sort_option,
            page_option,
            ..Default::default()
        })
            .await
    }
    pub async fn update_name(db: &DbConn, id: i32, name: String) -> Result<Model, DbErr> {
        let x = ActiveModel {
            id: Set(id),
            name: Set(name),
            ..Default::default()
        };
        x.update(db).await
    }
}


```


