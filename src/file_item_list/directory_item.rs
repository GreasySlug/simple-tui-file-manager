use std::path::PathBuf;

use crate::path_process::pathbuf_to_string_name;

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
    path: PathBuf,
    parent: PathBuf,
}

impl Directory {
    pub fn new(path: PathBuf) -> Self {
        let name = pathbuf_to_string_name(&path);
        let mut parent = path.clone();
        if !parent.pop() {
            // TODO: macos, windows, linux
            // parent = PathBuf::from("root");

            #[cfg(target_os = "windows")]
            {
                parent = PathBuf::from(r"C:\");
            }

            #[cfg(not(target_os = "windows"))]
            {
                paernt = PathBuf::from(r"/")
            }
        }
        Self { name, path, parent }
    }

    pub fn pathbuf(&self) -> &PathBuf {
        &self.path
    }

    pub fn parent(&self) -> &PathBuf {
        &self.parent
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::Directory;

    #[test]
    #[cfg(target_os = "windows")]
    fn root_parent_test() {
        let path = PathBuf::from(r"C:\");
        let dir = Directory::new(path);
        assert_eq!(dir.parent(), &PathBuf::from(r"C:\"));

        let path = PathBuf::from(r"C:\Users\");
        let dir = Directory::new(path);
        assert_eq!(dir.parent(), &PathBuf::from(r"C:\"));
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn directory_name_test() {
        let path = PathBuf::from("sample/test/");
        let dir = Directory::new(path);
        assert_eq!(dir.name(), "test");
    }
}
