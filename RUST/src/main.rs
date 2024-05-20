use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    id: i32,
    name: String,
}

async fn create_data() -> impl Responder {
    let mut conn = match Connection::open("mydb.db") {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Cade o banco?: {}", err);
            return HttpResponse::InternalServerError().body("Sem banco é foda");
        }
    };

    if let Err(err) = conn.execute(
        "CREATE TABLE IF NOT EXISTS my_table (id INTEGER PRIMARY KEY, name TEXT NOT NULL);",
        [],
    ) {
        eprintln!("Sem table: {}", err);
        return HttpResponse::InternalServerError().body("Não crio não");
    }

    let tx = match conn.transaction() {
        Ok(tx) => tx,
        Err(err) => {
            eprintln!("Não inseriu: {}", err);
            return HttpResponse::InternalServerError().body("Não mando não");
        }
    };

    {
        let mut stmt = match tx.prepare("INSERT INTO my_table (name) VALUES (?1)") {
            Ok(stmt) => stmt,
            Err(err) => {
                eprintln!("Xiiii: {}", err);
                return HttpResponse::InternalServerError().body("Foi não eim");
            }
        };

        for i in 0..10_000 {
            if let Err(err) = stmt.execute(params![format!("name {}", i)]) {
                eprintln!("Deu ruim,: {}", err);
                return HttpResponse::InternalServerError().body("Deu ruim!");
            }
        }
    }

    if let Err(err) = tx.commit() {
        eprintln!("Failed to commit transaction: {}", err);
        return HttpResponse::InternalServerError().body("Failed to commit transaction");
    }

    HttpResponse::Ok().body("Criado caraio")
}

async fn retrieve_data() -> impl Responder {
    let conn = match Connection::open("mydb.db") {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Conecto não: {}", err);
            return HttpResponse::InternalServerError().body("Sem banco parça");
        }
    };

    let mut stmt = match conn.prepare("SELECT id, name FROM my_table ORDER BY id LIMIT 10000") {
        Ok(stmt) => stmt,
        Err(err) => {
            eprintln!("Request paia: {}", err);
            return HttpResponse::InternalServerError().body("Arruma saporra");
        }
    };

    let my_structs_iter = match stmt.query_map([], |row| {
        Ok(MyStruct {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }) {
        Ok(iter) => iter,
        Err(err) => {
            eprintln!("Num foi: {}", err);
            return HttpResponse::InternalServerError().body("Sem map");
        }
    };

    let mut my_structs: Vec<MyStruct> = Vec::new();
    for my_struct in my_structs_iter {
        match my_struct {
            Ok(m) => my_structs.push(m),
            Err(err) => {
                eprintln!("Achei n: {}", err);
                return HttpResponse::InternalServerError().body("Ta sem registro");
            }
        }
    }

    HttpResponse::Ok().json(my_structs)
}

async fn clear_data() -> impl Responder {
    let conn = match Connection::open("mydb.db") {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Cade o banco?: {}", err);
            return HttpResponse::InternalServerError().body("Sem db é foda");
        }
    };

    if let Err(err) = conn.execute("DELETE FROM my_table", []) {
        eprintln!("Delete n deu: {}", err);
        return HttpResponse::InternalServerError().body("N consegui");
    }

    if let Err(err) = conn.execute("VACUUM", []) {
        eprintln!("Não limpo: {}", err);
        return HttpResponse::InternalServerError().body("Deu pra limpa não");
    }

    HttpResponse::Ok().body("Limpin")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/create", web::post().to(create_data))
            .route("/retrieve", web::get().to(retrieve_data))
            .route("/clear", web::post().to(clear_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
