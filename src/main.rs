use actix_web::{HttpServer,  App, Responder, HttpResponse, HttpRequest, web};
use actix_web::http::header::{HeaderValue, HeaderName};
use reqwest::{Client, header::HeaderMap, Response};

const WAKATIME_WEB:&str = "https://wakatime.com/";

async fn index(req: HttpRequest, client: web::Data<Client>)-> impl Responder{
    let path_to_req = req.path();

    // get url
    let mut url: String = String::from(WAKATIME_WEB);
    url.push_str(path_to_req);
    println!("URL: {}",url);
    
    //get headers
    let headers = req.headers().to_owned();
    let mut headers_for_req2 = HeaderMap::new();
    for key in headers.keys().cloned() {
        let key = key.to_string();
        let value: String = String::from(headers.get(key.clone()).unwrap().to_str().unwrap());

        headers_for_req2.insert(reqwest::header::HeaderName::from_bytes(key.as_bytes()).unwrap(),
            value.parse().unwrap());
    }

    let req2: Response = client.get(url).headers(headers_for_req2).send().await.unwrap();

    //setup response for replying
    let headers: &HeaderMap = req2.headers();
    let mut resp = HttpResponse::new(req2.status());
    for k in headers.keys(){
        let key = k.to_string();
        let value = String::from(headers.get(key.clone()).unwrap().to_str().unwrap());

        if key.to_lowercase() == "content-encoding"{
            continue;
        }

        resp.headers_mut().insert(HeaderName::from_bytes(key.as_bytes()).unwrap(),
            HeaderValue::from_bytes(value.as_bytes()).unwrap());
    }
    resp.headers_mut().insert(HeaderName::from_bytes(b"Access-Control-Allow-Origin").unwrap(),
        HeaderValue::from_bytes(b"*").unwrap());
    resp.headers_mut().insert(HeaderName::from_bytes(b"Content-Encoding").unwrap(),
        HeaderValue::from_bytes(b"Identity").unwrap());
    let body = req2.text().await.unwrap();
    resp.headers_mut().insert(HeaderName::from_bytes(b"Content-Length").unwrap(),
        HeaderValue::from_bytes(body.len().to_string().as_bytes()).unwrap());
    let resp = resp.set_body(body);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = match std::env::var("PORT"){
        Ok(x)=> x.parse::<u16>().unwrap(),
        Err(_)=> 7777
    };
    HttpServer::new(||{
        App::new()
            .app_data(web::Data::new(Client::new()))
            .route("/api/v1/{something}", web::get().to(index))
    })
    .bind(("0.0.0.0",port))?
    .run().await
}
