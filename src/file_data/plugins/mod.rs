/// The plugin for plain text
pub mod plain_text;
pub use plain_text::{
    PlainTextFromBinaryDataPluginTrait, PlainTextToBinaryDataPluginTrait,
};
/// The plugin for plain text
pub mod list_of_strings;
pub use list_of_strings::{
    ListOfStringsFromBinaryDataPluginTrait,
    ListOfStringsToBinaryDataPluginTrait,
};
#[cfg(feature = "font_support")]
/// The plugin for plain text
pub mod font;
#[cfg(feature = "font_support")]
pub use font::{FontFromBinaryDataPluginTrait, FontToBinaryDataPluginTrait};

#[cfg(feature = "image_support")]
/// The plugin for plain text
pub mod image_support;
#[cfg(feature = "image_support")]
pub use image_support::{DynImageFromBinaryDataPluginTrait, DynImageToBinaryDataPluginTrait};
#[allow(unused_imports)]
use super::BinaryData;

#[derive(Clone, Copy)]
/// All information required to extend [`BinaryData`]
///
/// For more information about how you may create your own [`BinaryData`] plugin, see [`this file`](plain_text)
pub struct BinaryDataPlugin<T: 'static> {
    /// The name of the plugin
    pub name: &'static str,
    /// The plugin itself
    pub plugin: &'static dyn BinaryDataPluginTrait<T>,
}
impl<T: 'static> core::fmt::Debug for BinaryDataPlugin<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryDataPlugin")
            .field("name", &self.name)
            .field("plugin", &"BinaryDataPluginTrait")
            .finish()
    }
}

impl<T> BinaryDataPlugin<T> {
    /// Create a new [`BinaryData`] plugin instance
    pub const fn new(
        name: &'static str,
        plugin: &'static dyn BinaryDataPluginTrait<T>,
    ) -> Self {
        Self {
            name,
            plugin,
        }
    }
}
/// The trait a `BinaryPlugin` struct needs to implement
pub trait BinaryDataPluginTrait<T>:
    std::marker::Send + std::marker::Sync
{
    /// Format the current struct if the format is supported by the current plugin, returning false if not.
    fn visualize_data(
        &self,
        f: &mut std::fmt::DebugStruct<'_, '_>,
        bin: &super::BinaryData<T>,
    ) -> bool;
}
