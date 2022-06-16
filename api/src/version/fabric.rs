use super::maven::get_download_url;

pub async fn get_fabric_version(version: String) -> Option<String> {
    let url = get_download_url(
        "https://maven.fabricmc.net".to_string(),
        "net.fabricmc".to_string(),
        "fabric-loader".to_string(),
        version,
    );

    let client = reqwest::Client::new();
    let res = client.get(&url).send().await;

    if res.unwrap().status() != 404 {
        return Some(url);
    } else {
        return None;
    }
}
