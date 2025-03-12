// use actix_web::{
//     App,
//     HttpResponse,
//     HttpServer,
//     Responder,
//     get,
//     post,
//     web,
// };

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// pub fn is_palindrome(x: i32) -> bool {
//     let vec_left_to_right = x.to_string().chars().collect::<Vec<char>>();
//     let vec_right_to_left =
// x.to_string().chars().rev().collect::<Vec<char>>();     for i in
// 0..vec_left_to_right.len() {         if vec_left_to_right[i] !=
// vec_right_to_left[i] {             return false;
//         }
//     }
//     for number in x {}
// }

use std::thread;

use actix_web::rt::time::sleep;

const NINTERATIONS: i32 = 10;

fn main() {
    // Spawn a new thread and capture its join handle.
    let handle = thread::spawn(|| {
        for i in 1..NINTERATIONS {
            println!(
                "Hello from the spawned thread: iteration {}, thread id: {:?}",
                i,
                thread::current().id()
            );
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    while !handle.is_finished() {
        println!("The spawned thread is still running.");
        thread::sleep(std::time::Duration::from_secs(1));
    }

    // Wait for the spawned thread to complete.
    // This avoid main thread to finish before the spawned thread.
    // It is a way to wait for the spawned thread to finish.
    handle.join().expect("The thread panicked");

    // Do some work in the main thread.
    for i in 1..5 {
        println!(
            "Hello from the main thread: iteration {}, thread id: {:?}",
            i,
            thread::current().id()
        );
    }

    println!("Main thread has finished.");
}
