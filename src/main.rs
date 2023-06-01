// use tokio_postgres::{Error, NoTls};
// // use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use actix_web::{get, web, App, HttpServer, Responder};

// // async fn connect() -> Result<(), Error> {
// //     // Set up the connection parameters
// //     let (client, connection) = tokio_postgres::connect(
// //         "host=efu17vdkxk.tpq4t695re.tsdb.cloud.timescale.com user=tsdbadmin dbname=tsdb",
// //         NoTls,
// //     )
// //     .await?;

// //     // Spawn a task to process the connection
// //     tokio::spawn(async move {
// //         if let Err(e) = connection.await {
// //             eprintln!("connection error: {}", e);
// //         }
// //     });

// //     // Execute a query
// //     let rows = client.query("SELECT * FROM your_table", &[]).await?;
// //     for row in rows {
// //         let value: i32 = row.get(0);
// //         println!("Value: {}", value);
// //     }

// //     Ok(())
// // }

// // async fn query_timescale() -> Result<impl Responder, Error> {
// //     // Set up the database connection parameters
// //     let (client, connection) = tokio_postgres::connect("host=efu17vdkxk.tpq4t695re.tsdb.cloud.timescale.com user=tsdbadmin dbname=tsdb", NoTls).await?;

// //     // Spawn a task to process the connection
// //     tokio::spawn(async move {
// //         if let Err(e) = connection.await {
// //             eprintln!("connection error: {}", e);
// //         }
// //     });

// //     // Execute a query
// //     let rows = client.query("SELECT * FROM conditions", &[]).await?;
// //     let result = rows.iter().map(|row| {
// //         let value: i32 = row.get(0);
// //         value.to_string()
// //     }).collect::<Vec<String>>().join(", ");

// //     Ok::<_, Error>(HttpResponse::Ok().body(result))
// //  }

// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     let (client, connection) =
//         tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;
//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("connection error: {}", e);
//         }
//     });

//     // Now we can execute a simple statement that just returns its parameter.
//     let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

//     // And then check that we got back the same string we sent over.
//     let value: &str = rows[0].get(0);
//     assert_eq!(value, "hello world");

//     Ok(())
// }

// // #[tokio::main]
// // -> std::io::Result<()>
// #[actix_web::main]
// async fn main() {
//     // // HttpServer::new(|| {
//     // //     App::new()
//     // //         .route("/query", web::get().to(query_timescale))
//     // // })
//     // // .bind("127.0.0.1:8080")?
//     // // .run()
//     // // .await
//     // HttpServer::new(|| App::new().service(greet))
//     //     .bind(("127.0.0.1", 8080))?
//     //     .run()
//     //     .await
//     actix_example_api::main();
// }

fn main() {
    actix_example_api::main();
}