use dotenv;
use tokio_postgres::types::Type;
use tokio_postgres::{Error, NoTls};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    let key = "connectURI";
    let value = dotenv::var(key).unwrap();

    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(value.as_str(), NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.

    let stmt = client
        .prepare(
            "SELECT *
        FROM pg_catalog.pg_tables
        WHERE schemaname != 'pg_catalog' AND 
            schemaname != 'information_schema';",
        )
        .await
        .unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();
    // And then check that we got back the same string we sent over.

    let get_table_desc_stmt = "select * from INFORMATION_SCHEMA.COLUMNS where table_name ='note';";

    for row in rows {
        for col in 0..row.columns().len() {
            let col_j = row.columns().get(col).unwrap();

            if col_j.type_().eq(&Type::NAME) {
                let val: Option<&str> = row.get(col);

                match val {
                    Some(x) => println!("{} {}", col_j.name(), x),
                    None => println!("{} {}", col_j.name(), "value is null"),
                }
            }
        }
        println!("")
    }

    Ok(())
}
