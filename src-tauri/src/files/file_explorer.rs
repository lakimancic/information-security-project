use std::path::PathBuf;
use std::fs;
use std::os::unix::fs::MetadataExt;
use chrono::{DateTime, Local};
use crate::files::errors::FilesError;
use crate::files::errors::FilesError::InvalidFilename;
use crate::files::file_entry::FileEntry;

pub struct FileExplorer {
    current_path: PathBuf,
}

impl FileExplorer {
    pub fn new() -> Self {
        Self { current_path: std::env::home_dir().get_or_insert_default().clone() }
    }

    pub fn change_dir(&mut self, dir: String) -> bool {
        self.current_path.push(dir);

        if self.current_path.is_dir() {
            true
        } else {
            self.current_path.pop();
            false
        }
    }
    
    pub fn get_current_path(&self) -> String { 
        self.current_path.to_str().unwrap_or("").to_string()
    }

    pub fn set_current_path(&mut self, path: String) -> bool {
        let new_path = PathBuf::from(path);
        if new_path.is_dir() {
            self.current_path = new_path;
            true
        } else {
            false
        }
    }

    pub fn go_back(&mut self) -> bool {
        self.current_path.pop()
    }

    pub fn list_entries(&mut self) -> Result<Vec<FileEntry>, FilesError> {
        let mut entries: Vec<FileEntry> = vec![];

        for entry in fs::read_dir(&self.current_path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;

            let filename = entry.file_name()
                .into_string()
                .map_err(|_| InvalidFilename)?;

            if metadata.is_dir() {
                entries.push(FileEntry {
                    filename,
                    size: None,
                    last_modified: String::new(),
                    file_type: Some("folder".into()),
                    type_long: Some("File Folder".into()),
                })
            }
            else {
                let (file_type, type_long) = FileExplorer::recognize_file_type(&filename);

                let last_modified: String = metadata.modified()
                    .map(|time| {
                        let datetime: DateTime<Local> = time.into();
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                    })
                    .unwrap_or_else(|_| "Unknown".to_string());

                entries.push(FileEntry {
                    filename,
                    size: Some(metadata.size()),
                    last_modified,
                    file_type,
                    type_long,
                })
            }
        }

        Ok(entries)
    }

    fn recognize_file_type(filename: &String) -> (Option<String>, Option<String>) {
        let extension = std::path::Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());

        match extension.as_deref() {
            Some("png") | Some("jpg") | Some("jpeg") | Some("bmp") | Some("gif") | Some("webp") => {
                (Some("image".into()), Some("Image File".into()))
            }
            Some("txt") | Some("md") | Some("log") | Some("json") | Some("rs") | Some("ts") => {
                (Some("text".into()), Some("Text Document".into()))
            }
            Some("zip") | Some("rar") | Some("7z") | Some("tar") | Some("gz") => {
                (Some("zip".into()), Some("Compressed Archive".into()))
            }
            Some("doc") | Some("docx") | Some("odt") => {
                (Some("doc".into()), Some("Word Document".into()))
            }
            Some("ppt") | Some("pptx") => {
                (Some("ppt".into()), Some("Presentation".into()))
            }
            Some("xls") | Some("xlsx") | Some("csv") => {
                (Some("xls".into()), Some("Spreadsheet".into()))
            }
            Some("pdf") => {
                (Some("pdf".into()), Some("PDF Document".into()))
            }
            Some("exe") | Some("msi") | Some("sh") | Some("bat") => {
                (Some("exe".into()), Some("Executable".into()))
            }
            Some("enc") => {
                (Some("enc".into()), Some("Encrypted File".into()))
            }
            _ => (None, Some("Unknown".into())),
        }
    }
}