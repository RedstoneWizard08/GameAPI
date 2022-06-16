use actix_web::{get, HttpRequest, HttpResponse};
use gameapi_api::versions::minecraft::{get_minecraft_versions, MinecraftVersionResult};
use serde::{Deserialize, Serialize};
use crate::util::{print_path, build_response};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TypeQuery {
    pub format: Option<String>,
    pub snapshots: Option<bool>,
}

#[get("/versions/{game}")]
async fn versions(req: HttpRequest) -> HttpResponse {
    let game = req.match_info().get("game").unwrap().to_lowercase();
    let des: TypeQuery = serde_urlencoded::from_str(&req.query_string()).unwrap();
    let format = des.format;
    let snapshots = des.snapshots;
    let mut vers: Option<MinecraftVersionResult> = None;
    let result: String;

    if game == "minecraft" {
        vers = Some(get_minecraft_versions().await);

        if !snapshots.is_none() && snapshots.unwrap() == false {
            let mut tmpvers = vers.unwrap();
            tmpvers.versions = tmpvers
                .versions
                .into_iter()
                .filter(|v| v.r#type != "snapshot" && !v.r#type.starts_with("old_"))
                .collect();
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

        print_path(req.clone(), "200");
    } else {
        print_path(req.clone(), "400");
        
        result = "That's not a supported game!".to_string();
    }

    return build_response(result.clone(), false);
}
