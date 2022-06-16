use actix_web::{get, HttpRequest, HttpResponse};
use gameapi_api::version::minecraft::get_minecraft_version;

use crate::util::{print_path, build_response};

#[get("/download/{game}/{version}/{side}")]
pub async fn download(req: HttpRequest) -> HttpResponse {
    let game = req.match_info().get("game").unwrap().to_lowercase();
    let side = req.match_info().get("side").unwrap().to_lowercase();
    let version = req.match_info().get("version").unwrap().to_lowercase();
    let result: String;

    if game == "minecraft" {
        let version = get_minecraft_version(version, side).await;

        if !version.is_none() {
            print_path(req.clone(), "200");

            result = version.unwrap();
        } else {
            print_path(req.clone(), "400");
            
            result = "That is not a supported version or side!".to_string();
        }
    } else {
        print_path(req.clone(), "400");
        
        result = "That is not a supported game!".to_string();
    }

    return build_response(result.clone(), result.starts_with("http"));
}
