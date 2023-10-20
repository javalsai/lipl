mod logger;
use actix_web::{
    get,
    http::{header, StatusCode},
    post,
    web::{self, Bytes, Redirect},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use lazy_static::lazy_static;
use logger::Logger;
use serde::Deserialize;
use std::{borrow::Cow, env, path::Path, sync::Mutex};
use urlencoding::{decode, encode};

lazy_static! {
    static ref REDIRECTED_LOGGER: Logger<'static> =
        Logger::new(Path::new("log/redirected")).unwrap();
    static ref DEFAULT_LOGGER: Logger<'static> = Logger::new(Path::new("log/default")).unwrap();
    static ref CONSOLE_LOGGER_COLOR: Mutex<bool> = Mutex::new(false);
    static ref RICK: String = "/rd/".to_string()
        + &encode(
            &"https://www.youtube.com/watch?v=dQw4w9WgXcQ"
                .chars()
                .rev()
                .collect::<String>()
        );
}

const RESPONSE: &[u8] = include_bytes!("../assets/tracker.html");

fn log_req(text: String, stderr: Option<String>) {
    let logger_color_handle = match CONSOLE_LOGGER_COLOR.lock() {
        Ok(handle) => Some(handle),
        Err(_) => None,
    };
    let logger_color: bool = match &logger_color_handle {
        Some(handle) => **handle,
        None => false,
    };

    println!("{}\n", if logger_color { "\x1b[40m" } else { "" });
    println!("{}", text);
    if stderr.is_some() {
        eprintln!("\x1b[1;31m{}", stderr.unwrap());
    }
    print!("\x1B[0m");

    if logger_color_handle.is_some() {
        *logger_color_handle.unwrap() = !logger_color
    }
}
fn generic_req_log(req: HttpRequest, extra: Option<String>) {
    let logger = &DEFAULT_LOGGER;

    let mut log_data = logger::serialize_req(&req);
    if extra.is_some() {
        log_data.push_str("\n\x1b[30m");
        log_data.push_str(&extra.unwrap())
    }

    let mut log_err = None;

    match logger.log(&log_data) {
        Ok(_) => {}
        Err(err) => {
            log_err = Some(format!("Error logging to file: {:?}", err));
        }
    };

    log_req(log_data, log_err);
}

#[get("/rd")]
async fn unknown_redirect() -> impl Responder {
    Redirect::to(&*RICK).permanent()
}

#[derive(Deserialize, Debug)]
struct RedirectQuery {
    n: Option<String>,
}
#[get("/rd/{uri}")]
async fn redirect(
    req: HttpRequest,
    param: web::Path<(String,)>,
    query: web::Query<RedirectQuery>,
) -> HttpResponse {
    if query.n.is_some() {
        let uri = param.into_inner().0;
        generic_req_log(req, Some("{ \"disabled\": true }".to_string()));
        return HttpResponse::Found()
            .insert_header((
                header::LOCATION,
                decode(&uri)
                    .unwrap_or(Cow::Borrowed(&RICK))
                    .chars()
                    .rev()
                    .collect::<String>()
                    .to_string(),
            ))
            .finish();
    } else {
        return HttpResponse::Ok().content_type("text/html").body(RESPONSE);
    }
}
#[post("/rd/{uri}")]
async fn data_post(req: HttpRequest, bytes: Bytes) -> HttpResponse {
    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => generic_req_log(req, Some(text)),
        Err(err) => generic_req_log(
            req,
            Some(format!(
                "{{ \"err\": \"{}\", \"response-bytes\": \"{:?}\" }}",
                err, bytes
            )),
        ),
    }

    return HttpResponse::Ok().content_type("text/html").body(RESPONSE);
}

async fn fallback(req: HttpRequest) -> HttpResponse {
    generic_req_log(req, None);
    HttpResponse::build(StatusCode::from_u16(404).unwrap()).body("Not Found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let listen_to: (&str, u16) = match args.len() {
        2 => ("0.0.0.0", args.get(1).unwrap().to_string().parse().unwrap()),
        3 => (
            args.get(1).unwrap(),
            args.get(2).unwrap().to_string().parse().unwrap(),
        ),
        _ => panic!("\x1B[1;31mErr, invalid arg number.\nUsage: lipl [listen address] port\x1B[0m"),
    };

    println!("Running on http://{}:{}", listen_to.0, listen_to.1);

    HttpServer::new(|| {
        App::new()
            .service(unknown_redirect)
            .service(redirect)
            .service(data_post)
            .default_service(web::route().to(fallback))
    })
    .bind(listen_to)?
    .run()
    .await
}
