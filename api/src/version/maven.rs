pub fn get_download_url(
    base_url: String,
    group_id: String,
    artifact_id: String,
    version: String,
) -> String {
    return format!(
        "{}/{}/{}/{}/{}-{}.jar",
        base_url,
        group_id.replace(".", "/"),
        artifact_id,
        version,
        artifact_id,
        version
    );
}
