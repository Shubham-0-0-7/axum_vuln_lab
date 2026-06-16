use axum::{
    extract::Query,
    response::Html,
};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct UrlParams{
    pub url: String,
}

pub async fn ssrf_page()->Html<&'static str>{
    Html(include_str!("../templates/ssrf.html"))
}

pub async fn fetch_url(Query(params): Query<UrlParams>)-> String{
    match reqwest::get(&params.url).await{
        Ok(res)=>{
            let status = res.status();
            match res.text().await{
                Ok(body)=>format!("status: {}\n\nbody:\n{}",status,body),
                Err(err)=>format!("status: {}\n\nerror: {}\n",status,err),
            }
        }
        Err(err)=>format!("error making request to {}:{}", params.url, err),
    }
}