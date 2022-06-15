use super::maven::{RawMavenVersionResult, MavenVersionResult};

pub async fn get_forge_versions() -> MavenVersionResult {
    let client = reqwest::Client::new();
    let res = client
        .get("https://maven.minecraftforge.net/net/minecraftforge/forge/maven-metadata.xml")
        .send()
        .await;

    let resp = res.unwrap().text().await.unwrap();
    let raw = xmltojson::to_json(&resp).expect("Something went wrong.");
    let serialized = serde_json::to_string(&raw).unwrap();
    let des: RawMavenVersionResult = serde_json::from_str(&serialized).unwrap();
    return des.metadata;
}
