use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftVersion {
    pub id: String,
    pub r#type: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftLatestVersion {
    pub release: String,
    pub snapshot: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftVersionResult {
    pub latest: MinecraftLatestVersion,
    pub versions: Vec<MinecraftVersion>,
}

pub async fn get_minecraft_versions() -> MinecraftVersionResult {
    let client = reqwest::Client::new();
    let res = client
        .get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .send()
        .await;

    let resp = res.unwrap();
    let text = resp.text().await.unwrap();

    return serde_json::from_str(&text).unwrap();
}
