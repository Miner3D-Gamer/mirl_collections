use crate::file_data::plugins::PlainTextFromBinaryDataPluginTrait;

use super::{
    super::{BinaryData, GenericDataType},
    BinaryDataPlugin, BinaryDataPluginTrait,
};
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// The plugin struct for handling plain text
pub struct DynImageBinaryDataPlugin;

/// The static position for [`DynImageBinaryDataPlugin`] as to allow for dyn
pub static DYN_IMAGE_BINARY_DATA_PLUGIN: DynImageBinaryDataPlugin =
    DynImageBinaryDataPlugin;

impl BinaryDataPluginTrait<GenericDataType> for DynImageBinaryDataPlugin {
    fn visualize_data(
        &self,
        f: &mut std::fmt::DebugStruct<'_, '_>,
        bin: &BinaryData<GenericDataType>,
    ) -> bool {
        if !matches!(bin.data_type, GenericDataType::Text) {
            return false;
        }
        f.field("raw_data@DynImage", &bin.to_str());

        true
    }
}
/// The trait, implemented for [`BinaryData`]
pub trait DynImageFromBinaryDataPluginTrait {
    /// Convert the raw bytes into an `image::DynamicImage` instance
    ///
    /// # Errors
    /// When unable to load the image from memory
    fn to_image(
        &self,
    ) -> Result<image::DynamicImage, Box<dyn core::error::Error>>;
}
/// The trait, implemented for [`BinaryData`]
pub trait DynImageToBinaryDataPluginTrait {
    /// Create a [`BinaryData`] from a string
    fn from_image(img: image::DynamicImage) -> Self;
}
impl<T> DynImageFromBinaryDataPluginTrait for BinaryData<T> {
    fn to_image(
        &self,
    ) -> Result<image::DynamicImage, Box<dyn core::error::Error>> {
        // Decode the raw bytes as an image
        let img = image::load_from_memory(&self.raw_data)?;

        Ok(img)
    }
}
impl DynImageToBinaryDataPluginTrait for BinaryData<GenericDataType> {
    fn from_image(img: image::DynamicImage) -> Self {
        Self {
            raw_data: img.into_bytes(),
            data_type: GenericDataType::Image,
        }
    }
}
impl DynImageToBinaryDataPluginTrait for BinaryData<String> {
    fn from_image(img: image::DynamicImage) -> Self {
        Self {
            raw_data: img.into_bytes(),
            data_type: "raw_data@Image".to_string(), // TODO: Get image format
        }
    }
}


inventory::submit! {
    BinaryDataPlugin {
        name: "dyn_image",
        plugin: &DYN_IMAGE_BINARY_DATA_PLUGIN,
    }
}
