use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct MyStruct {
    id: i32,
    name: String,
    email: String,
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
        "CREATE TABLE IF NOT EXISTS my_table (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT NOT NULL);",
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
        let mut stmt = match tx.prepare("INSERT INTO my_table (name, email) VALUES (?1, ?2)") {
            Ok(stmt) => stmt,
            Err(err) => {
                eprintln!("Xiiii: {}", err);
                return HttpResponse::InternalServerError().body("Foi não eim");
            }
        };

        for i in 0..10_000 {
            if let Err(err) = stmt.execute(params![format!("name {}", i), format!("email{}@example.com", i)]) {
                eprintln!("Deu ruim: {}", err);
                return HttpResponse::InternalServerError().body("Deu ruim!");
            }
        }
    }

    if let Err(err) = tx.commit() {
        eprintln!("Failed to commit transaction: {}", err);
        return HttpResponse::InternalServerError().body("Deu ruim na transação!");
    }

    HttpResponse::Ok().body("Dados inseridos com sucesso!")
}

async fn retrieve_data() -> impl Responder {
    let conn = match Connection::open("mydb.db") {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Falha ao abrir o banco de dados: {}", err);
            return HttpResponse::InternalServerError().body("Erro ao conectar ao banco de dados");
        }
    };

    let mut stmt = match conn.prepare("SELECT id, name, email FROM my_table") {
        Ok(stmt) => stmt,
        Err(err) => {
            eprintln!("Falha ao preparar a consulta SQL: {}", err);
            return HttpResponse::InternalServerError().body("Erro ao preparar a consulta SQL");
        }
    };

    let data_iter = match stmt.query_map([], |row| {
        Ok(MyStruct {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    }) {
        Ok(data_iter) => data_iter,
        Err(err) => {
            eprintln!("Erro ao executar a consulta SQL: {}", err);
            return HttpResponse::InternalServerError().body("Erro ao executar a consulta SQL");
        }
    };

    let mut data = Vec::new();
    for result in data_iter {
        match result {
            Ok(record) => data.push(record),
            Err(err) => {
                eprintln!("Erro ao processar registro: {}", err);
                return HttpResponse::InternalServerError().body("Erro ao processar registros");
            }
        }
    }

    HttpResponse::Ok().json(data)
}

async fn clear_data() -> impl Responder {
    let conn = match Connection::open("mydb.db") {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Falha ao abrir o banco de dados: {}", err);
            return HttpResponse::InternalServerError().body("Erro ao conectar ao banco de dados");
        }
    };

    if let Err(err) = conn.execute("DELETE FROM my_table", []) {
        eprintln!("Erro ao deletar registros: {}", err);
        return HttpResponse::InternalServerError().body("Erro ao deletar registros");
    }

    if let Err(err) = conn.execute("VACUUM", []) {
        eprintln!("Erro ao limpar banco de dados: {}", err);
        return HttpResponse::InternalServerError().body("Erro ao limpar banco de dados");
    }

    HttpResponse::Ok().body("Dados deletados e banco de dados limpo")
}

async fn count_data() -> impl Responder {
    let conn = match Connection::open("mydb.db") {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Falha ao abrir o banco de dados: {}", err);
            return HttpResponse::InternalServerError().body("Erro ao conectar ao banco de dados");
        }
    };

    let count: i64 = match conn.query_row("SELECT COUNT(*) FROM my_table", [], |row| row.get(0)) {
        Ok(count) => count,
        Err(err) => {
            eprintln!("Erro ao contar registros: {}", err);
            return HttpResponse::InternalServerError().body("Erro ao contar registros");
        }
    };

    HttpResponse::Ok().json(json!({ "count": count }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/create", web::post().to(create_data))
            .route("/retrieve", web::get().to(retrieve_data))
            .route("/clear", web::post().to(clear_data))
            .route("/count", web::get().to(count_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
