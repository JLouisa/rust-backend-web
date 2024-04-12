use libsql::Builder;

async fn get_items() -> Result<HttpResponse, Error> {
    dotenv().expect(".env file not found");

    let db_file = env::var("LOCAL_DB").unwrap();

    let db = Builder::new_local(db_file).build().await?;

    let conn = db.connect().unwrap();

    let mut results = conn.query("SELECT * FROM items", ()).await.unwrap();

    let mut items: Vec<T> = Vec::new();

    while let Some(row) = results.next().await.unwrap() {
        let item: Item = Item {
            task: row.get(0).unwrap(),
        };
        items.push(item);
    }

    Ok(HttpResponse::Ok().json(items))
}
