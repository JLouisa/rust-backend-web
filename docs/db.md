setup sqlite db v1

```rust
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use actix_web::{get, middleware::Logger, web, App, Error, HttpResponse, HttpServer, Responder};

pub struct SqliteDB {
    pub db: Pool<Sqlite>,
}

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_sqlite_url.as_str())
        .await
        // .expect("Database connection failed");

    let pool = match pool {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            eprintln!(
                "If the database has not been created, please run \n $ sqlx database setup \n"
            );
            panic!("Database connection failed");
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .app_data(SqliteDB { db: pool.clone() })
            .service(health)
    })
        .bind((address, port))?
    .run()
    .await
```

SQLITE Schema migration v1

```rust
    sqlx::migrate!("./migrations/sqlx")
        .run(&database_sqlx.0)
        .await
        .expect("Migration failed");
```
