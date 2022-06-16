use super::maven::get_download_url;

pub async fn get_quilt_version(version: String) -> Option<String> {
    let url = get_download_url(
        "https://maven.quiltmc.org/repository/release".to_string(),
        "org.quiltmc".to_string(),
        "quilt-loader".to_string(),
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
