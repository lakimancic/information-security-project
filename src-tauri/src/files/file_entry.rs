use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub filename: String,
    pub size: Option<u64>,
    pub last_modified: String,
    pub file_type: Option<String>,
    pub type_long: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileExplore {
    pub files: Vec<FileEntry>,
    pub pwd: String,
}