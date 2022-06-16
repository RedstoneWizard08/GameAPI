use super::maven::get_download_url;

pub async fn get_forge_version(version: String) -> Option<String> {
    let url = get_download_url(
        "https://maven.minecraftforge.net".to_string(),
        "net.minecraftforge".to_string(),
        "forge".to_string(),
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
