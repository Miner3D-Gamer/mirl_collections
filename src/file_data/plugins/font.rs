use super::{
    super::{BinaryData, GenericDataType},
    BinaryDataPlugin, BinaryDataPluginTrait,
};
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// The plugin struct for handling plain text
pub struct FontBinaryDataPlugin;

/// The static position for [`FontBinaryDataPlugin`] as to allow for dyn
pub static FONT_BINARY_DATA_PLUGIN: FontBinaryDataPlugin = FontBinaryDataPlugin;

impl BinaryDataPluginTrait<GenericDataType> for FontBinaryDataPlugin {
    fn visualize_data(
        &self,
        f: &mut std::fmt::DebugStruct<'_, '_>,
        bin: &BinaryData<GenericDataType>,
    ) -> bool {
        if !matches!(bin.data_type, GenericDataType::Font) {
            return false;
        }
        f.field("raw_data@Font", &bin.to_font());

        true
    }
}
/// The trait, implemented for [`BinaryData`]
pub trait FontFromBinaryDataPluginTrait {
    /// Convert the raw bytes into a fontdue font
    ///
    /// # Errors
    /// If the data is not in utf8 format
    fn to_font(self) -> Result<fontdue::Font, &'static str>;
}
/// The trait, implemented for [`BinaryData`]
pub trait FontToBinaryDataPluginTrait {
    /// Create a [`BinaryData`] from a string
    fn from_string(string: String) -> Self;
}
impl<T> FontFromBinaryDataPluginTrait for BinaryData<T> {
    fn to_font(self) -> Result<fontdue::Font, &'static str> {
        fontdue::Font::from_bytes(
            self.raw_data,
            fontdue::FontSettings::default(),
        )
    }
}
impl FontToBinaryDataPluginTrait for BinaryData<GenericDataType> {
    fn from_string(string: String) -> Self {
        Self {
            raw_data: string.into_bytes(),
            data_type: GenericDataType::Text,
        }
    }
}
impl FontToBinaryDataPluginTrait for BinaryData<String> {
    fn from_string(string: String) -> Self {
        Self {
            raw_data: string.into_bytes(),
            data_type: "string@utf8".to_string(),
        }
    }
}

inventory::submit! {
    BinaryDataPlugin {
        name: "font",
        plugin: &FONT_BINARY_DATA_PLUGIN,
    }
}
