```
cargo check
cargo clean
cargo build --target x86_64-unknown-linux-musl --release
```
```shell
    let vec = prelude::AuthUser::find().all(connection).await.expect("");

    log::info!("{:?}",vec);

    let vec1 = prelude::AuthUser::find().from_raw_sql(
        Statement::from_sql_and_values(
            DatabaseBackend::MySql,
            "select*from auth_user",
            [])).all(master()).await.expect("");
     log::info!("{:?}",vec1);

    let vec1 = UserVO::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::MySql,
        "select*from auth_user",
        [])).all(connection).await.expect("");
    log::info!("{:?}",vec1);
    let vec1 :Vec<JsonValue>= <JsonValue as FromQueryResult>::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::MySql,
        "select*from auth_user",
        [])).all(connection).await.expect("");
    log::info!("{:?}",vec1);

    let vec1 = prelude::AuthUser::find().from_raw_sql(Statement::from_sql_and_values(
        DatabaseBackend::MySql,
        "select*from auth_user",
        [])).paginate(connection,3).fetch_page(1).await.expect("");
    log::info!("{:?}",vec1);

    let vec = connection.query_all(
        Statement::from_sql_and_values(
            DatabaseBackend::MySql, "select*from auth_user limit 1", [])).await.expect("");
    log::info!("{:?}",vec);

    let vec = connection.execute(
        Statement::from_sql_and_values(
            DatabaseBackend::MySql, "select*from auth_user limit 1", [])).await.expect("");
    log::info!("{:?}",vec.rows_affected());
    
    
    #[derive(Clone, Debug, FromQueryResult)]
pub struct UserVO {
    pub user_id: i64,
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub remark: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub sex: Option<i8>,
    pub avatar: Option<String>,
    pub status: i8,
    pub login_ip: Option<String>,
    pub login_time: Option<DateTime>,
    pub create_user_id: Option<i64>,
    pub create_time: DateTime,
    pub update_user_id: Option<i64>,
    pub update_time: DateTime,
    pub deleted: bool,
}
```