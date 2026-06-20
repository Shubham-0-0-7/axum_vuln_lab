use axum::{
    extract::Query,
    response::Html,
};
use rusqlite::Connection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InvoiceParams{
    pub id:i32,
}

pub async fn idor_page()->Html<&'static str>{
    Html(include_str!("../templates/idor.html"))
}

pub async fn init_db(){
    let conn = Connection::open("db/app.db").expect("failed to open database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS invoices (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            amount REAL NOT NULL,
            description TEXT NOT NULL,
            date TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create table");

    let count:i64 = conn.query_row("SELECT COUNT(*) FROM invoices", [], |row| row.get(0)).unwrap_or(0);
    if count == 0 {
         conn.execute(
            "INSERT INTO invoices (user_id, amount, description, date) VALUES 
            (1, 1500.00, 'Enterprise Server Hosting Plan', '2026-06-01'),
            (1, 120.50, 'Domain Name Registration (.com)', '2026-06-05'),
            (2, 450.00, 'Graphic Design Consultation Fees', '2026-06-10')",
            [],
        ).expect("Failed to seed invoices table");
        println!("[+] Seeded invoices table in db/app.db");
    }
}

pub async fn get_invoice(Query(params):Query<InvoiceParams>)->String{
    let conn = match Connection::open("db/app.db"){
        Ok(c)=>c,
        Err(err)=>return format!("database error: {}", err),
    };
    let mut stmt = match 
    conn.prepare("SELECT id, user_id, amount, description, date FROM invoices WHERE id = ?"){
        Ok(s)=>s,
        Err(err)=>return format!("sql prepare error: {}", err),
    };
    let invoice = stmt.query_row([params.id], |row|{
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
        ))
    });
    match invoice{
        Ok((id, user_id, amount, description, date))=>{ 
            format!("invoice id: {}\n owner user id: {}\namount: ${:.2}\ndescription: {}\ndate: {}", id, user_id, amount, description, date)
        },
        Err(rusqlite::Error::QueryReturnedNoRows)=>format!("error: invoice with id {} not found", params.id),
        Err(err)=>format!("database error occured: {}", err),
    }
}