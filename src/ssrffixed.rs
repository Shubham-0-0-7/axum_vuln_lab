use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use std::net::IpAddr;
use reqwest::Url;

#[derive(Deserialize)]
pub struct UrlParams{
    pub url:String,
}

fn is_private(ip:IpAddr)->bool{
    match ip{
        IpAddr::V4(ip4)=>{
            ip4.is_loopback() ||
            ip4.is_private() ||
            ip4.is_link_local() ||
            ip4.is_unspecified() || 
            ip4.is_multicast()
        }
        IpAddr::V6(ip6)=>{
            ip6.is_loopback() ||
            ip6.is_unspecified() ||
            ip6.is_multicast() || 
            (ip6.segments()[0] & 0xff00) == 0xfe00 || 
            (ip6.segments()[0] & 0xfe00) == 0xfc00
        }
    }
}

pub async fn fetch_url_safe(Query(params):Query<UrlParams>)-> impl IntoResponse{
    let url = match Url::parse(&params.url){
        Ok(u) => u,
        Err(err) => return (StatusCode::BAD_REQUEST, format!("Invalid URL: {}", err)).into_response(),
    };
    let host = match url.host_str(){
        Some(h)=>h,
        None=>return (StatusCode::BAD_REQUEST, "missing host in url".to_string()).into_response(),
    };
    let port = url.port().unwrap_or_else(|| {
        match url.scheme(){
            "https"=>443,
            _ => 80,
        }
    });

    let addr = format!("{}:{}", host, port);
    let resolved_ips = match tokio::net::lookup_host(&addr).await{
        Ok(iter)=>iter.map(|socket_addr| socket_addr.ip()).collect::<Vec<IpAddr>>(), 
        Err(err)=> return (StatusCode::BAD_REQUEST, format!("dns resolution failed for {}:{}", host, err)).into_response(),
    };

    if resolved_ips.is_empty(){
        return (StatusCode::BAD_REQUEST, format!("could not resolve host {}", host)).into_response();
    }

    for ip in resolved_ips{
        if is_private(ip){
            return (StatusCode::FORBIDDEN, format!("access denied! requested ip addr {} is a restricted private/loopback address!", ip)).into_response();
        }
    }

    match reqwest::get(url.as_str()).await {
        Ok(res) => {
            let status = res.status();
            match res.text().await {
                Ok(body) => (StatusCode::OK, format!("STATUS: {}\n\nBODY:\n{}", status, body)).into_response(),
                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("STATUS: {}\n\nError reading body: {}", status, err)).into_response(),
            }
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error making request to {}: {}", url.as_str(), err)).into_response(),
    }
}