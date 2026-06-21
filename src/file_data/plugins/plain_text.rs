use super::{
    super::{BinaryData, GenericDataType},
    BinaryDataPlugin, BinaryDataPluginTrait,
};
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// The plugin struct for handling plain text
pub struct PlainTextBinaryDataPlugin;

/// The static position for [`PlainTextBinaryDataPlugin`] as to allow for dyn
pub static PLAIN_TEXT_BINARY_DATA_PLUGIN: PlainTextBinaryDataPlugin =
    PlainTextBinaryDataPlugin;

impl BinaryDataPluginTrait<GenericDataType> for PlainTextBinaryDataPlugin {
    fn visualize_data(
        &self,
        f: &mut std::fmt::DebugStruct<'_, '_>,
        bin: &BinaryData<GenericDataType>,
    ) -> bool {
        if !matches!(bin.data_type, GenericDataType::Text) {
            return false;
        }
        f.field("raw_data@PlainText", &bin.to_str());

        true
    }
}
/// The trait, implemented for [`BinaryData`]
pub trait PlainTextFromBinaryDataPluginTrait {
    /// Convert the raw bytes to a &str (if valid UTF-8)
    ///
    /// # Errors
    /// If the data is not in utf8 format
    fn to_str(&self) -> Result<&str, std::str::Utf8Error>;

    /// Convert the raw bytes to a String (if valid UTF-8)
    ///
    /// # Errors
    /// If the data is not in utf8 format
    fn to_string(self) -> Result<String, std::string::FromUtf8Error>;
}
/// The trait, implemented for [`BinaryData`]
pub trait PlainTextToBinaryDataPluginTrait {
    /// Create a [`BinaryData`] from a string
    fn from_string(string: String) -> Self;
}
impl<T> PlainTextFromBinaryDataPluginTrait for BinaryData<T> {
    fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        str::from_utf8(&self.raw_data)
    }
    fn to_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.raw_data)
    }
}
impl PlainTextToBinaryDataPluginTrait for BinaryData<GenericDataType> {
    fn from_string(string: String) -> Self {
        Self {
            raw_data: string.into_bytes(),
            data_type: GenericDataType::Text,
        }
    }
}
impl PlainTextToBinaryDataPluginTrait for BinaryData<String> {
    fn from_string(string: String) -> Self {
        Self {
            raw_data: string.into_bytes(),
            data_type: "string@utf8".to_string(),
        }
    }
}

inventory::submit! {
    BinaryDataPlugin {
        name: "plain_text",
        plugin: &PLAIN_TEXT_BINARY_DATA_PLUGIN,
    }
}
