use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    version: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MavenVersionInfo {
    latest: String,
    release: String,
    versions: VersionInfo,
    last_updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MavenVersionResult {
    pub group_id: String,
    pub artifact_id: String,
    pub versioning: MavenVersionInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawMavenVersionResult {
    pub metadata: MavenVersionResult,
}
