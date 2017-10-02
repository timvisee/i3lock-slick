extern crate image;

use std::fs::File;

use self::image::DynamicImage;

use err::{Error, Result};
use img::Img;

/// Editable image struct.
pub struct ImgEdit {
    img: DynamicImage
}

impl ImgEdit {
    /// Constructor from a dynamic image.
    pub fn from(img: DynamicImage) -> Self {
        ImgEdit {
            img
        }
    }

    /// Load the given image.
    pub fn load(img: &Img) -> Result<Self> {
        // Load the image
        match image::open(img.path_buf()) {
            Ok(load) => Ok(Self::from(load)),
            Err(_) => Err(Error::new(
                "Failed to load image for editing"
            ))
        }
    }

    /// Change the editable image into a dynamic image, that is used by the image crate.
    ///
    /// Although this consumes the image, it can be converted back using `ImgEdit::from(out)`.
    pub fn into_img(self) -> DynamicImage {
        self.img
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
