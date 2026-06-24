use axum::{
    extract::Query,
    http::StatusCode,
    http::HeaderMap,
    response::IntoResponse,
};
use rusqlite::Connection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InvoiceParams{
    pub id:i32,
}

pub async fn get_invoice_safe(Query(params):Query<InvoiceParams>, headers: HeaderMap,)-> impl IntoResponse{
    let user_id = match headers.get("x-user-id"){
        Some(val)=>{
            match val.to_str(){
                Ok(s)=>match s.parse::<i32>(){
                    Ok(uid)=>uid,
                    Err(_) => return (StatusCode::BAD_REQUEST, "Invalid X-User-Id header format".to_string()).into_response(),
                },
                Err(_) => return (StatusCode::BAD_REQUEST, "Invalid X-User-Id header encoding".to_string()).into_response(),
            }
        }
        None => return (StatusCode::UNAUTHORIZED, "Missing authentication header X-User-Id. Please select a user session.".to_string()).into_response(),
    };

    let conn = match Connection::open("db/app.db"){
        Ok(c)=>c,
        Err(err)=>return (StatusCode::INTERNAL_SERVER_ERROR, format!("database conenction error: {}", err)).into_response(),
    };

    let mut stmt = match conn.prepare("SELECT id, user_id, amount, description, date FROM invoices WHERE id = ?"){
        Ok(s)=>s,
        Err(err)=>return (StatusCode::INTERNAL_SERVER_ERROR, format!("sql prepare error: {}", err)).into_response(),
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

    match invoice {
        Ok((id, owner_id, amount, description, date))=>{
            if owner_id != user_id{
                return(
                    StatusCode::FORBIDDEN,
                    format!("Access Denied: You (User ID: {}) do not have permission to view Invoice ID {} (owned by User ID: {})", user_id, id, owner_id)
                ).into_response();
            }
            (
                StatusCode::OK,
                format!(
                    "Invoice ID: {}\nOwner User ID: {}\nAmount: ${:.2}\nDescription: {}\nDate: {}",
                    id, owner_id, amount, description, date
                )
            ).into_response()
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            (StatusCode::NOT_FOUND, format!("Error: Invoice with ID {} not found.", params.id)).into_response()
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error occurred: {}", err)).into_response()
        }
    }
}