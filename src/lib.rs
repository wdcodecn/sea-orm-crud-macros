extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, DeriveInput, Expr, Lit};

#[proc_macro_derive(SeaOrmCrud, attributes(sea_orm))]
pub fn sea_orm_crud_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident; // 获取实体结构体的名称
    let attrs: &Vec<Attribute> = &input.attrs; // 获取实体结构体的名称

    // 打印生成的代码 (通过 ToTokens trait)
    let generated_code1 = format!("{:?}", attrs.clone());
    // eprintln!("Generated code:\n{:?}", generated_code1);

    // 找到 sea_orm 属性并提取其中的 table_name
    // 查找 `sea_orm` 属性
    let mut table_name = "".to_string();
    let mut comment = quote! {None};
    let mut schema_name = quote! { None };
    let mut table_iden = false;
    // let mut rename_all: Option<CaseStyle> = None;

    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("sea_orm"))
        .try_for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("comment") {
                    let name: Lit = meta.value()?.parse()?;
                    comment = quote! { Some(#name) };
                } else if meta.path.is_ident("table_name") {
                    let name: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = &name {
                        let table_name_str = lit_str.value(); // 去掉双引号
                        table_name = table_name_str.clone();
                        // eprintln!("Found table_name: {}", table_name_str); // 打印去除双引号后的值
                    }
                } else if meta.path.is_ident("schema_name") {
                    let name: Lit = meta.value()?.parse()?;
                    schema_name = quote! { Some(#name) };
                } else if meta.path.is_ident("table_iden") {
                    table_iden = true;
                } else {
                    // Reads the value expression to advance the parse stream.
                    // Some parameters, such as `primary_key`, do not have any value,
                    // so ignoring an error occurred here.
                    let _: Option<Expr> = meta.value().and_then(|v| v.parse()).ok();
                }

                Ok(())
            })
        })
        .unwrap();

    // let table_name = get_table_name(&input); // 获取表名（来自实体结构体）
    let update_name = format_ident!("New{struct_name}");
    let table_name = format_ident!("{table_name}");

    // 获取结构体的字段
    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("UpdateFields only supports structs"),
    };

    // 遍历结构体字段，生成 `update_field` 方法
    let field_methods = fields.iter().map(|field| {
        let field_name = &field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let method_name = format_ident!("get_by_{field_name}");
        let update_method_name = format_ident!("update_{field_name}");

        if *field_name == "id" {
            quote! {}
        }else {

            // let schema = format_ident!("{}", model.to_string().to_snake_case());
            let struct_name = format_ident!("{field_name}");
            let x =to_pascal_case(field_name.to_string());
            let pascal_field_name = format_ident!("{x}");


            // eprintln!("Found field: {:?}", struct_name);
            let update_name = format_ident!("update_{field_name}");


            // 生成每个字段的 update_field 方法
            quote! {
                pub async fn #method_name(db: &DbConn, #field_name:  #field_type ,

                    sort_option: Option<SortOption>,
                    page_option: Option<PageOption>,
                ) -> Result<(Vec<Model>, ItemsAndPagesNumber), DbErr> {
                    Self::find_by_option_model(db, OptionModel {
                        #field_name: Some(#field_name),
                        sort_option,
                        page_option,
                        ..Default::default()
                    }).await
                }

                pub async fn #update_method_name(db: &DbConn, id: i32, #field_name:  #field_type) -> Result<Model, DbErr> {
                    let x = ActiveModel {
                        id: Set(id),
                        #field_name: Set(#field_name),
                        ..Default::default()
                    };
                    x.update(db).await
                }
            }
        }


    });

    // 遍历结构体字段，生成 `update_field` 方法
    let find_by_field_methods = fields.iter().map(|field| {
        let field_name = &field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let x = to_pascal_case(field_name.to_string());
        let pascal_field_name = format_ident!("{x}");

        quote! {

           if let Some(n) = x.#field_name {
                query = query.filter(Column::#pascal_field_name.eq(n));
            }
        }
    });

    // 遍历结构体字段，生成 `update_field` 方法
    let option_field_methods = fields.iter().map(|field| {
        let field_name = &field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        quote! {
            pub #field_name: Option<#field_type>,
        }
    });

    // 构建生成的 CRUD 方法代码
    let expanded = quote! {

            #[derive(Debug)]
            pub struct PageOption {
                page: u64,
                page_size: u64,
            }

            /// 默认分页
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

            /// 默认排序 CreatedAt 倒序
            impl Default for SortOption {
                fn default() -> Self {
                    SortOption {
                        name: Column::CreatedAt,
                        desc: true,
                    }
                }
            }

            #[derive(Debug,Default)]
            pub struct OptionModel {

                #(#option_field_methods)*
                pub sort_option: Option<SortOption>,
                pub page_option: Option<PageOption>,
            }

            use sea_orm::ActiveValue::Set;
            use sea_orm::QueryOrder;
            use sea_orm::ItemsAndPagesNumber;

            pub struct Service {}

            impl Service {
                pub async fn insert_one(db: &DbConn, x: ActiveModel) -> Result<Model, DbErr> {
                    x.insert(db).await
                }
                pub async fn get_one_by_id(db: &DbConn, id: i32) -> Result<Option<Model>, DbErr> {
                    Entity::find_by_id(id).one(db).await
                }

                pub async fn find_by_option_model(db: &DbConn, x: OptionModel) -> Result<(Vec<Model>, ItemsAndPagesNumber), DbErr> {
                    let mut query = Entity::find();


                      #(#find_by_field_methods)*

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


                #(#field_methods)*

            }

            /*#[derive(DeriveIntoActiveModel)]
            pub struct #update_name {
                #(#field_methods)*
            }
    */
        };

    // 打印生成的代码 (通过 ToTokens trait)
    let generated_code = expanded.to_string();
    // eprintln!("Generated code:\n{}", generated_code);

    TokenStream::from(expanded)
}

fn to_pascal_case(s: String) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            let first_char = chars.next().unwrap_or_default().to_uppercase();
            let rest: String = chars.collect();
            format!("{}{}", first_char, rest)
        })
        .collect::<String>()
}

