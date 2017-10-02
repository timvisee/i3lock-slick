extern crate image;

use std::fs::File;
use std::path::PathBuf;

use self::image::DynamicImage;

use super::Result;
use super::Error;

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

    pub fn edit(&self) -> Result<ImgEditor> {
        ImgEditor::load(self)
    }

    pub fn path_buf(&self) -> &PathBuf {
        &self.file
    }
}

/// Image editor struct.
pub struct ImgEditor {
    img: DynamicImage
}

impl ImgEditor {
    /// Load the given image.
    pub fn load(img: &Img) -> Result<Self> {
        // Load the image
        match image::open(img.path_buf()) {
            Ok(load) => Ok(
                ImgEditor {
                    img: load
                }
            ),
            Err(_) => Err(Error::new(
                "Failed to load image for editing"
            ))
        }
    }

    pub fn blur(&mut self) {
        self.img = self.img.blur(4.0).brighten(-20);
    }

    /// Save the edited image.
    pub fn save(self, img: &Img) -> Result<File> {
        // Open target file
        let mut img_file = File::create(&img.path_buf());
        if img_file.is_err() {
            return Err(Error::new("Failed create file to save the processed image to"));
        }

        // Save the image
        let img_out = self.img.save(img_file.as_mut().unwrap(), image::PNG);
        if img_out.is_err() {
            return Err(Error::new("Failed to save processed image"));
        }

        Ok(img_file.unwrap())
    }
}
