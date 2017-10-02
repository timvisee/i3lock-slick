extern crate image;

use std::path::PathBuf;

use err::{Result};
use img::ImgEdit;

/// Image struct.
pub struct Img {
    file: PathBuf
}

impl Img {
    pub fn new(file: &PathBuf) -> Self {
        Img {
            file: file.clone()
        }
    }

//    pub fn from<'a>(file: &'a str) -> Self {
//        Self::new(&PathBuf::from(file))
//    }

    pub fn edit(&self) -> Result<ImgEdit> {
        ImgEdit::load(self)
    }

    pub fn path_buf(&self) -> &PathBuf {
        &self.file
    }
}
