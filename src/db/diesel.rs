users::table.load(&mut connection);

let client = libsql_client::Client::from_config(libsql_client::Config {
    url: url::Url::parse("/home/jlouisa/development/sqlite.sql").unwrap(),
    auth_token: None,
})
.await
.unwrap();
