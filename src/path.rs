use sqlx::{Pool,Postgres};
use be_erp::{ update_account, login_account, tambah_akun};
use tide::{Server};

pub fn path(app: &mut Server<Pool<Postgres>>){

app.at("update").put(update_account);
app.at("login").post(login_account);
app.at("daftar").post(tambah_akun);
}
