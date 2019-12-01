use std::path::PathBuf;
use crate::{
    HttpdArgs,
    Result,
    mqtt::{self, Mqtt},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex}
};
use async_std::{fs as async_fs, io as async_io, task as async_task};
use log::{debug, info};


type ServerState = Arc<Mutex<Mqtt>>;

pub fn start(args: HttpdArgs, mut mqtt: Mqtt) -> Result<()> {

    let bus = Arc::new(Mutex::new(bus::Bus::new(255)));
    mqtt.register_subscriber({
        let bus = bus.clone();
        move |event| {
            if let mqtt::Event::Message(msg) = event {
                bus.lock().unwrap().broadcast(msg.clone());
            }
        }
    });
    super::ws_server::start(args.httpd_host.clone(), bus)?;

    let mut app = tide::with_state(Arc::new(Mutex::new(mqtt)));
    app.at("/").get(|_| redirect("index.html"));
    app.at("/api/publish").post(publish);
    match args.httpd_webapp_dir.clone() {
        Some(path) => app.at("/*").get(move |req: tide::Request<ServerState>| {
            let path = path.join(req.uri().path().trim_start_matches('/'));
            serve_asset_from_path(path)
        }),
        None => {
            let assets: HashMap<&str, Vec<u8>> = include!(concat!(env!("OUT_DIR"), "/assets.rs"));
            debug!("embedded assets: {:#?}", assets.keys());
            app.at("/*").get(move |req: tide::Request<ServerState>| {
                let path = req.uri().path();
                serve_embedded_asset(path.to_string(), assets.get(path).cloned())
            })
        },
    };

    async_task::block_on(
        async {
            let addr = format!("{}:{}", args.httpd_host, args.httpd_port);
            info!("listen on addr: {}", addr);
            app.listen(addr).await?;
            Ok(())
        },
    )
}


async fn redirect(target: impl AsRef<str>) -> tide::Response {
    tide::Response::new(307).set_header("Location", target)
}


async fn serve_asset_from_path(path: PathBuf) -> tide::Response {
    debug!("serve asset from path: {}", path.display());
    match async_fs::metadata(&path).await.ok() {
        Some(meta) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            debug!("{} Content-Type: {:?}", path.display(), mime);
            let file = async_fs::File::open(path).await.unwrap();
            let reader = async_io::BufReader::new(file);
            tide::Response::new(200)
                .set_header("Content-Length", meta.len().to_string())
                .body(reader)
                .set_mime(mime)
        }
        None => {
            debug!("asset not found: {}", path.display());
            tide::Response::new(404).body_string("Not found".into())
        }
    }
}


async fn serve_embedded_asset(path: String, asset: Option<Vec<u8>>) -> tide::Response {
    debug!("serve embedded asset: {}", path);
    match asset {
        Some(asset) => {
            let ext = path.split('.').last().unwrap_or("");
            let mime = mime_guess::from_ext(ext).first_or_octet_stream();
            debug!("{} Content-Type: {:?}", path, mime);
            tide::Response::new(200)
                .set_header("Content-Length", asset.len().to_string())
                .body(async_io::Cursor::new(asset))
                .set_mime(mime)
        },
        None => {
            debug!("embedded asset not found: {}", path);
            tide::Response::new(404).body_string("Not found".into())
        },
    }
}

async fn publish(mut req: tide::Request<ServerState>) -> tide::Response {
    #[derive(serde::Deserialize)]
    struct Payload {
        topic: String,
        message: String,
    }

    match req.body_json::<Payload>().await {
        Ok(payload) => {
            let mqtt = &mut req.state().lock().unwrap();
            match mqtt.publish(payload.topic, payload.message) {
                Ok(_) => tide::Response::new(201),
                Err(err) => tide::Response::new(500).body_string(err.to_string()),
            }
        },
        Err(err) => tide::Response::new(400).body_string(err.to_string()),
    }
}
