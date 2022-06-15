use actix_web::{get, App, HttpServer, Responder, HttpRequest};
use gameapi_api::versions::{minecraft::{get_minecraft_versions, MinecraftVersionResult}, fabric::get_fabric_versions, quilt::get_quilt_versions, maven::MavenVersionResult, forge::get_forge_versions};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct TypeQuery {
    pub format: Option<String>,
    pub snapshots: Option<bool>,
}

#[get("/versions/{game}")]
async fn versions(req: HttpRequest) -> impl Responder {
    let game = req.match_info().get("game").unwrap().to_lowercase();
    let des: TypeQuery = serde_urlencoded::from_str(&req.query_string()).unwrap();
    let format = des.format;
    let snapshots = des.snapshots;

    println!("\x1b[34m[\x1b[36mGET\x1b[34m] \x1b[32m/versions/{game}?{} \x1b[35mHTTP/1.1 \x1b[1;32m200 OK\x1b[0m", req.query_string());
    
    let mut vers: Option<MinecraftVersionResult> = None;
    let result: String;

    if game == "minecraft" {
        vers = Some(get_minecraft_versions().await);

        if !snapshots.is_none() && snapshots.unwrap() == false {
            let mut tmpvers = vers.unwrap();
            tmpvers.versions = tmpvers.versions.into_iter().filter(|v| v.r#type != "snapshot" && !v.r#type.starts_with("old_")).collect();
            vers = Some(tmpvers);
        }
    }

    if !vers.is_none() {
        if format.is_none() {
            result = serde_json::to_string_pretty(&vers).unwrap() as String;
        } else {
            let fmt = format.unwrap();
    
            if fmt == "json".to_string() || fmt == "json5".to_string() {
                result = serde_json::to_string_pretty(&vers).unwrap() as String;
            } else if fmt == "yaml".to_string() || fmt == "yml".to_string() {
                result = serde_yaml::to_string(&vers).unwrap() as String;
            } else if fmt == "toml".to_string() {
                result = toml::to_string_pretty(&vers).unwrap() as String;
            } else {
                result = serde_json::to_string_pretty(&vers).unwrap() as String;
            }
        }
    } else {
        result = "That's not a supported game!".to_string();
    }

    return result;
}

#[get("/versions/{game}/{loader}")]
pub async fn loader(req: HttpRequest) -> impl Responder {
    let game = req.match_info().get("game").unwrap().to_lowercase();
    let loader = req.match_info().get("loader").unwrap().to_lowercase();
    let des: TypeQuery = serde_urlencoded::from_str(&req.query_string()).unwrap();
    let format = des.format;

    println!("\x1b[34m[\x1b[36mGET\x1b[34m] \x1b[32m/versions/{game}/{loader}?{} \x1b[35mHTTP/1.1 \x1b[1;32m200 OK\x1b[0m", req.query_string());
    
    let mut vers: Option<MavenVersionResult> = None;
    let result: String;

    if game == "minecraft" {
        if loader == "fabric" {
            vers = Some(get_fabric_versions().await);
        } else if loader == "quilt" {
            vers = Some(get_quilt_versions().await);
        } else if loader == "forge" {
            vers = Some(get_forge_versions().await);
        }
    }

    if !vers.is_none() {
        if format.is_none() {
            result = serde_json::to_string_pretty(&vers).unwrap() as String;
        } else {
            let fmt = format.unwrap();
    
            if fmt == "json".to_string() || fmt == "json5".to_string() {
                result = serde_json::to_string_pretty(&vers).unwrap() as String;
            } else if fmt == "yaml".to_string() || fmt == "yml".to_string() {
                result = serde_yaml::to_string(&vers).unwrap() as String;
            } else if fmt == "toml".to_string() {
                result = toml::to_string_pretty(&vers).unwrap() as String;
            } else {
                result = serde_json::to_string_pretty(&vers).unwrap() as String;
            }
        }
    } else {
        result = "That's not a supported game or loader!".to_string();
    }

    return result;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(versions)
            .service(loader)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
