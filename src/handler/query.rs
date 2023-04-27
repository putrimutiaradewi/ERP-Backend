use serde::Deserialize;
use sqlx::PgPool;
use tide::{Request, Response, Body, http};
use crate::ws_response;



#[derive(serde::Serialize, Debug ,Deserialize)]
struct Account {
    username :String,
    alamat :String,
    telephone : String,
    password :String,
}
#[derive(serde::Serialize, Debug ,Deserialize)]
struct Ul {
    username :String,
    password :String,
}
#[derive(serde::Serialize, Debug ,Deserialize)]
struct LoginResult {
    status :String,
    info :String,
    
}

pub async fn tambah_akun (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Account = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("INSERT INTO users (username, alamat, password, telephone) VALUES ($1,$2,sha256($3),$4);")
     .bind(param.username)
     .bind(param.alamat)
     .bind(param.password.as_bytes())
     .bind(param.telephone)
     
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil register")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal register")
        }

     }

}

pub async fn update_account (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Ul = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("UPDATE users SET password=sha256($2) WHERE username=$1")
     .bind(param.username)
     .bind(param.password.as_bytes())
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil Update")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal Update ")
        }

     }
}


pub async fn login_account (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Ul = req.body_json().await?;
    let pool = req.state();
    let mut resp = Response::new(http::StatusCode::Ok);

    if let Ok(record) = sqlx::query!(
        "SELECT username FROM users WHERE username = $1  and password = sha256($2::text::bytea)",
        param.username, param.password,
    ).fetch_one(pool).await{

        let ret = LoginResult{
            status: "Ok".to_string(),
            info: "Login berhasil".to_string(),

        };
        resp.set_status(200);
        resp.set_body(Body::from_json(&ret)?);
    } else {
        let ret = serde_json::json!(
            {
                "status": "Error",
                "info": "Username/password Invalid"
            }
        );
        resp.set_status(http::StatusCode::Ok);
        resp.set_body(Body::from_json(&ret)?);
    }
    Ok(resp)
}



