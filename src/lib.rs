use std::sync::Arc;

use async_trait::async_trait;
use jni::{objects::JClass, JNIEnv};
use once_cell::sync::Lazy;
use serde_json::json;
use tokio::{io::AsyncWriteExt, runtime::Runtime, sync::Mutex};
use util::{load_tls_config, JSON};

pub mod util;

const ROOT_CA: &str = r#"
-----BEGIN CERTIFICATE-----
MIIC7DCCAdQCCQCsmUkJHta+MjANBgkqhkiG9w0BAQsFADA4MREwDwYDVQQDDAh0
ZXN0LmxvYzELMAkGA1UEBhMCVVMxFjAUBgNVBAcMDVNhbiBGcmFuc2lzY28wHhcN
MjMwNjA4MTEwNjE1WhcNMjQwNTI5MTEwNjE1WjA4MREwDwYDVQQDDAh0ZXN0Lmxv
YzELMAkGA1UEBhMCVVMxFjAUBgNVBAcMDVNhbiBGcmFuc2lzY28wggEiMA0GCSqG
SIb3DQEBAQUAA4IBDwAwggEKAoIBAQDhycenlVPwfUlFTAYPFqC4NVpHXEsoEmEw
fvnIGmymXx6T1hBLUAUT47Srek/9C3VwpzY/6ahy4auljR8JZNoMNOVmrW+80gfw
6TmdVQmWtE2Q+xNMZZBdA3Cj7Cc/mu0yq10zJe608DT9gbVqbTS1C/Gum71rQ3U2
fuaQCrP76c6bYvFpXjxwUWGW1gA6EF1ShvT9dZSl4R+JspaIRWEZ4ubmBY1x5aBn
q37+AAejwdVq8YU3TETEz5/eLWaxeclEpPyl8Lfz0xfAXYifgdL85LxTRA0JjLAz
J6IxXVJM10Jh9DbJFYCLch4fIz1HM27CnNBi1KwDXlU/KFFQwJqxAgMBAAEwDQYJ
KoZIhvcNAQELBQADggEBAIocT5N95Z1Mc33VR6itBlriMSjJ+Xb4eXY8mD73vdXy
wISyL6rrOnojlrxFiVcvtqn5/Bvtn9QTjIWdjsZrgH1oVvs9saa002O8UIi9R3ks
txKU4tAt9NeM8tuJkb1s/xRUVNL8fx+n5+ldp1hKI9Y01hjXI1+SuXFCS8zF+YM3
NZsY7sW3FbrSjzXYAGJRHRMJ57AYMzsBUqq8GmpEPE6/OUcLmLtoUFKNc4/+A+bS
CupQ5hxZg7yIanHIJqxNQWmmjZR+XrDHw/Q/WMHv7QH4yaI5WkTxUkLvvMFSBidT
Skv9bD6TA/ucHVPsOYwzy2hQmRkUGr+3ahDdPLI48GY=
-----END CERTIFICATE-----
"#;

pub struct Handler {
    pub conn: Option<Conn>, //to close
}

impl Handler {
    pub fn new() -> Self {
        Self { conn: None }
    }
}

#[derive(Clone)]
pub struct Env {
    pub rt: Arc<Box<Runtime>>,
    pub handler: Arc<Mutex<Handler>>,
}
impl Env {
    pub fn new() -> Self {
        let rt = Arc::new(Box::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .unwrap(),
        ));
        let handler = Arc::new(Mutex::new(Handler::new()));
        Self { rt, handler }
    }
}

static ENV: Lazy<std::sync::Mutex<Env>> = Lazy::new(|| {
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("quincrash")
            .with_max_level(log::LevelFilter::Debug),
    );
    std::sync::Mutex::new(Env::new())
});

#[async_trait]
pub trait RawReader {
    async fn read(&mut self, buf: &mut [u8]) -> tokio::io::Result<usize>;
}

#[async_trait]
pub trait RawWriter {
    async fn write(&mut self, buf: &[u8]) -> tokio::io::Result<usize>;
    async fn shutdown(&mut self);
}

pub struct WrapQuinnReader {
    inner: quinn::RecvStream,
}

#[async_trait]
impl RawReader for WrapQuinnReader {
    async fn read(&mut self, buf: &mut [u8]) -> tokio::io::Result<usize> {
        match self.inner.read(buf).await? {
            Some(v) => Ok(v),
            None => Ok(0),
        }
    }
}

pub struct WrapQuinnWriter {
    inner: quinn::SendStream,
    closed: bool,
}

#[async_trait]
impl RawWriter for WrapQuinnWriter {
    async fn write(&mut self, buf: &[u8]) -> tokio::io::Result<usize> {
        self.inner.write_all(buf).await?;
        self.inner.flush().await?;
        Ok(buf.len())
    }
    async fn shutdown(&mut self) {
        // not check close will be crash
        // if self.closed {
        //     return;
        // }
        self.closed = true;
        _ = self.inner.shutdown().await;
    }
}

pub fn wrap_quinn_w(
    send: quinn::SendStream,
    recv: quinn::RecvStream,
) -> (
    Box<dyn RawReader + Send + Sync>,
    Box<dyn RawWriter + Send + Sync>,
) {
    let rxa = Box::new(WrapQuinnReader { inner: recv });
    let txa = Box::new(WrapQuinnWriter {
        inner: send,
        closed: false,
    });
    (rxa, txa)
}

#[derive(Clone)]
pub struct Conn {
    pub reader: Arc<Mutex<Box<dyn RawReader + Send + Sync>>>,
    pub writer: Arc<Mutex<Box<dyn RawWriter + Send + Sync>>>,
}

async fn connect() -> Conn {
    let mut options = JSON::new();
    options.insert(String::from("tls_ca"), json!(ROOT_CA));
    let options = Arc::new(options);
    let addr = "192.168.1.9:3100".parse().unwrap();
    let domain = "test.loc";
    let runtime = quinn::default_runtime().unwrap();
    let conn = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    let mut endpoint =
        quinn::Endpoint::new(quinn::EndpointConfig::default(), None, conn, runtime).unwrap();
    let tls = load_tls_config(Arc::new(String::from(".")), &options).unwrap();
    endpoint.set_default_client_config(quinn::ClientConfig::new(tls));
    let conn = endpoint.connect(addr, domain).unwrap().await.unwrap();
    let (mut send, recv) = conn.open_bi().await.unwrap();
    _ = send.write_all(&"abc".as_bytes()).await.unwrap(); // server is not reponse data, if not write first
    let (reader, writer) = wrap_quinn_w(send, recv);
    let conn_handler = Conn {
        reader: Arc::new(Mutex::new(reader)),
        writer: Arc::new(Mutex::new(writer)),
    };
    let conn_run = conn_handler.clone();
    tokio::spawn(async move { read(conn_run).await });
    conn_handler
}

async fn read(conn: Conn) {
    log::info!("read_loop is starting");
    let mut reader = conn.reader.lock().await;
    let mut buffer = vec![0u8; 1024];
    loop {
        match reader.read(&mut buffer).await {
            Ok(n) => {
                if n < 1 {
                    break;
                }
                log::info!("receive {} bytes", n)
            }
            Err(_) => break,
        }
    }
    // close twice
    _ = conn.writer.lock().await.shutdown().await;
    log::info!("read_loop is stopped");
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_quinncrash_Bind_start(_: JNIEnv, _: JClass) {
    let env = ENV.lock().unwrap();
    let handler = env.handler.clone();
    env.rt.block_on(async move {
        let send = connect().await;
        handler.lock().await.conn = Some(send);
    });
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_quinncrash_Bind_close(_: JNIEnv, _: JClass) {
    let env = ENV.lock().unwrap();
    let handler = env.handler.clone();
    env.rt.block_on(async move {
        let mut handler = handler.lock().await;
        match &mut handler.conn {
            Some(c) => {
                log::info!("conn is shutdown");
                let mut conn = c.writer.lock().await;
                _ = conn.shutdown().await;
            }
            None => {
                log::info!("conn is not found");
            }
        };
        handler.conn = None;
    });
}
