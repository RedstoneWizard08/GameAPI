use actix_web::{get, HttpRequest, HttpResponse};
use gameapi_api::version::{
    fabric::get_fabric_version, forge::get_forge_version, quilt::get_quilt_version,
};
use crate::util::{print_path, build_response};

#[get("/download/loader/{game}/{loader}/{version}")]
pub async fn download_loader(req: HttpRequest) -> HttpResponse {
    let game = req.match_info().get("game").unwrap().to_lowercase();
    let loader_param = req.match_info().get("loader").unwrap().to_lowercase();
    let version = req.match_info().get("version").unwrap().to_lowercase();
    let result: String;

    if game == "minecraft" {
        if loader_param == "fabric" {
            let version_result = get_fabric_version(version).await;

            if !version_result.is_none() {
                print_path(req.clone(), "200");

                result = version_result.unwrap();
            } else {
                print_path(req.clone(), "400");

                result = "That is not a supported version!".to_string();
            }
        } else if loader_param == "quilt" {
            let version_result = get_quilt_version(version).await;

            if !version_result.is_none() {
                print_path(req.clone(), "200");

                result = version_result.unwrap();
            } else {
                print_path(req.clone(), "400");

                result = "That is not a supported version!".to_string();
            }
        } else if loader_param == "forge" {
            let version_result = get_forge_version(version).await;

            if !version_result.is_none() {
                print_path(req.clone(), "200");
                
                result = version_result.unwrap();
            } else {
                print_path(req.clone(), "400");

                result = "That is not a supported version!".to_string();
            }
        } else {
            print_path(req.clone(), "400");

            result = "That is not a supported loader!".to_string();
        }
    } else {
        print_path(req.clone(), "400");

        result = "That is not a supported game!".to_string();
    }

    return build_response(result.clone(), result.starts_with("http"));
}
