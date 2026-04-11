// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Sandbox File Operations
// ═══════════════════════════════════════════════════════════════════════════════
//  File system operations within sandbox
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// File type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    File,
    Directory,
    Symlink,
}

/// File information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// File path
    pub path: String,
    /// File name
    pub name: String,
    /// File type
    pub file_type: FileType,
    /// File size in bytes
    pub size: u64,
    /// Last modified timestamp
    pub modified: Option<String>,
    /// Permissions (Unix mode)
    pub mode: Option<u32>,
}

impl FileInfo {
    /// Check if it's a file
    pub fn is_file(&self) -> bool {
        self.file_type == FileType::File
    }

    /// Check if it's a directory
    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
    }

    /// Get file extension
    pub fn extension(&self) -> Option<&str> {
        self.name.rsplit('.').next()
    }
}

/// File content for read/write
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    /// File path
    pub path: String,
    /// Content (text or base64 for binary)
    pub content: String,
    /// Whether content is base64 encoded
    pub is_base64: bool,
    /// Encoding for text files
    pub encoding: Option<String>,
}

impl FileContent {
    /// Create text file content
    pub fn text(path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            content: content.into(),
            is_base64: false,
            encoding: Some("utf-8".to_string()),
        }
    }

    /// Create binary file content (base64 encoded)
    pub fn binary(path: impl Into<String>, base64_content: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            content: base64_content.into(),
            is_base64: true,
            encoding: None,
        }
    }
}

/// Directory listing request
#[derive(Debug, Clone, Serialize)]
pub struct ListDirRequest {
    /// Directory path
    pub path: String,
    /// Include hidden files
    #[serde(default)]
    pub include_hidden: bool,
    /// Recursive listing
    #[serde(default)]
    pub recursive: bool,
}

impl ListDirRequest {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            include_hidden: false,
            recursive: false,
        }
    }

    pub fn with_hidden(mut self) -> Self {
        self.include_hidden = true;
        self
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }
}

/// File write request
#[derive(Debug, Clone, Serialize)]
pub struct WriteFileRequest {
    /// File path
    pub path: String,
    /// File content
    pub content: String,
    /// Create parent directories
    #[serde(default = "default_true")]
    pub create_dirs: bool,
    /// Overwrite if exists
    #[serde(default = "default_true")]
    pub overwrite: bool,
}

fn default_true() -> bool { true }

impl WriteFileRequest {
    pub fn new(path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            content: content.into(),
            create_dirs: true,
            overwrite: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_info() {
        let info = FileInfo {
            path: "/home/user/test.py".to_string(),
            name: "test.py".to_string(),
            file_type: FileType::File,
            size: 1024,
            modified: None,
            mode: Some(0o644),
        };

        assert!(info.is_file());
        assert!(!info.is_dir());
        assert_eq!(info.extension(), Some("py"));
    }

    #[test]
    fn test_file_content_text() {
        let content = FileContent::text("test.py", "print('hello')");
        assert!(!content.is_base64);
        assert_eq!(content.encoding, Some("utf-8".to_string()));
    }

    #[test]
    fn test_list_dir_request() {
        let req = ListDirRequest::new("/home")
            .with_hidden()
            .recursive();

        assert!(req.include_hidden);
        assert!(req.recursive);
    }
}
