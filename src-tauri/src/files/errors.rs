#[derive(Debug, thiserror::Error)]
pub enum FilesError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid UTF-8 in filename")]
    InvalidFilename,
    #[error("File Explorer Internal Error")]
    ExplorerInternalError,
}

#[derive(serde::Serialize)]
#[serde(tag = "name", content = "message")]
#[serde(rename_all = "camelCase")]
enum ErrorName {
    Io(String),
    InvalidFilename(String),
    FileExplorerInternalError(String),
}

impl serde::Serialize for FilesError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let message = self.to_string();
        let name = match self {
            Self::Io(_) => ErrorName::Io(message),
            Self::InvalidFilename => ErrorName::InvalidFilename(message),
            FilesError::ExplorerInternalError => ErrorName::FileExplorerInternalError(message),
        };
        name.serialize(serializer)
    }
}
