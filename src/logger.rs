use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use actix_web::{http::header::HeaderMap, HttpRequest};
use chrono::Local;

pub struct Logger<'a> {
    path: &'a Path,
}

pub struct LHttpRequest<'a>(&'a HttpRequest);
impl LHttpRequest<'_> {
    pub fn serialize(&self) -> String {
        let headers = self.0.headers();
        let conn_info = self.0.connection_info();

        let url = format!(
            "url: {:?}, version: {:?}",
            self.0.uri().to_string(),
            format!("{:?}", self.0.version())
        );
        let agent = format!("agent: {:?}", _get_header(headers, "User-Agent"));
        let ips = format!(
            "peer: {:?}, real_ip: {:?}",
            conn_info.peer_addr().unwrap_or_default(),
            conn_info.realip_remote_addr().unwrap_or_default()
        );
        let acceptance = format!(
            "accept-lang: {:?}, accept-encoding: {:?}",
            _get_header(headers, "Accept-Language"),
            _get_header(headers, "Accept-Encoding")
        );
        let cookie = format!(
            "cookie: {:?}, uir: {:?}",
            _get_header(headers, "Cookie"),
            _get_header(headers, "Upgrade-Insecure-Requests")
        );

        format!("{}\n{}\n{}\n{}\n{}", url, agent, ips, acceptance, cookie)
    }
}

impl Logger<'_> {
    pub fn new(path: &Path) -> std::io::Result<Logger> {
        fs::create_dir_all(path)?;
        Ok(Logger { path })
    }

    pub fn log(&self, data: &str) -> std::io::Result<()> {
        let datename = Local::now().format("%Y-%m-%d_%H:%M:%S").to_string();
        let logfilename = self.path.join(datename);

        let mut logfile = File::create(logfilename)?;

        logfile.write_all(data.as_bytes())?;

        Ok(())
    }
}

pub fn serialize_req(req: &HttpRequest) -> String {
    LHttpRequest(req).serialize()
}

fn _get_header<'a>(headers: &'a HeaderMap, header: &str) -> &'a str {
    match headers.get(header) {
        Some(val) => val.to_str().unwrap_or_default(),
        None => "",
    }
}
