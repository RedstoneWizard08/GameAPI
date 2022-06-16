use std::collections::HashMap;

use either::Either;
use serde::{Deserialize, Serialize};

use crate::versions::minecraft::get_minecraft_versions;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OSRule {
    pub name: Option<String>,
    pub version: Option<String>,
    pub arch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JvmArgRule {
    pub action: Option<String>,
    pub os: Option<OSRule>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameArgRule {
    pub action: Option<String>,
    pub features: Option<HashMap<String, bool>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JvmArgs {
    pub rules: Option<Vec<JvmArgRule>>,

    #[serde(with = "either::serde_untagged")]
    pub value: Either<Vec<String>, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameArgs {
    pub rules: Option<Vec<GameArgRule>>,

    #[serde(with = "either::serde_untagged")]
    pub value: Either<Vec<String>, String>,
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct MinecraftVersionArgs {
//     pub game: Vec<Either<String, GameArgs>>,
//     pub jvm: Vec<Either<JvmArgs, String>>,
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: usize,
    pub total_size: usize,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Download {
    pub sha1: String,
    pub size: usize,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
    pub component: String,
    pub major_version: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactDownload {
    pub path: String,
    pub sha1: String,
    pub size: usize,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LibraryDownload {
    pub downloads: HashMap<String, ArtifactDownload>,
    pub name: String,
    pub rules: Option<Vec<JvmArgRule>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoggingFileInfo {
    pub id: String,
    pub sha1: String,
    pub size: usize,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoggingInfo {
    pub argument: String,
    pub file: LoggingFileInfo,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftVersionInfo {
    // Had to disable this because it threw errors.
    // pub arguments: MinecraftVersionArgs,
    pub asset_index: AssetIndex,
    pub assets: String,
    pub compliance_level: usize,
    pub downloads: HashMap<String, Download>,
    pub id: String,
    pub java_version: JavaVersion,
    pub libraries: Vec<LibraryDownload>,
    pub logging: HashMap<String, LoggingInfo>,
    pub main_class: String,
    pub minimum_launcher_version: usize,
    pub release_time: String,
    pub time: String,
    pub r#type: String,
}

pub async fn get_minecraft_version(version: String, side: String) -> Option<String> {
    let versions = Some(get_minecraft_versions().await).unwrap().versions;
    let version = versions.into_iter().find(|v| v.id == version);
    let url = version.unwrap().url;
    let client = reqwest::Client::new();
    let res = client.get(url).send().await;

    let resp = res.unwrap();
    let text = resp.text().await.unwrap();

    let result: MinecraftVersionInfo = serde_json::from_str(&text).unwrap();
    let download: &Download;

    if side == "client" {
        download = result.downloads.get("client").unwrap();
    } else if side == "server" {
        download = result.downloads.get("server").unwrap();
    } else {
        return None;
    }

    return Some(download.url.to_owned());
}
