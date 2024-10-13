use crate::File;

impl File {
    pub fn from(path_: String, content_: String) -> Self {
        File {
            path: path_,
            content: content_,
        }
    }
}
