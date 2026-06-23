/// The default plugins provided for [`BinaryData`]
pub mod plugins;
use plugins::BinaryDataPlugin;

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[cfg(feature = "image")]

inventory::collect!(BinaryDataPlugin<GenericDataType>);

inventory::collect!(BinaryDataPlugin<String>);

#[cfg_attr(
    feature = "mirl_derive",
    mirl_derive::derive_all(read_only = true, zerocopy = false)
)]
/// This struct hold the raw data in memory, often received from a file, to be converted/used somewhere else
///
/// ---
///
/// The generic `TD` determines what data types are allowed.
///
/// This generic is usually set to [`GenericDataType`] for generic formats, or [`String`] for dynamic formats, or a custom empty enum for hyper specialized formats
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BinaryData<DT = GenericDataType> {
    /// Raw data
    pub raw_data: Vec<u8>,
    /// What the raw data is expected to be.
    ///
    /// This is not automatically determined by this struct but instead set by items using this struct.
    pub data_type: DT,
}
impl std::fmt::Display for BinaryData<GenericDataType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut struct_format = f.debug_struct("BinaryData");

        for plugin in inventory::iter::<BinaryDataPlugin<GenericDataType>> {
            if plugin.plugin.visualize_data(&mut struct_format, self) {
                return struct_format.finish();
            }
        }
        if matches!(self.data_type, GenericDataType::Bytes) {
            struct_format.field("raw_data", self.as_bytes());
        }
        // TODO: There has to be a better way to convey the error
        #[allow(clippy::print_in_format_impl)]
        match self.data_type {
            GenericDataType::Bytes => struct_format.finish(),
            GenericDataType::Color => {
                println!(
                    "`{:?}` is not supported by default. Please follow the following steps:\n 1. import `mirl_graphics` with the `binary_data_plugin` flag\n 2. import `mirl_graphics::prelude::ColorBinaryDataPluginTrait`",
                    self.data_type
                );
                Err(core::fmt::Error)
            }
            _ => {
                println!(
                    "`{:?}` is not supported by default. To be displayed. Please import a plugin that adds support for this type.",
                    self.data_type
                );
                Err(core::fmt::Error)
            }
        }
    }
}
// impl BinaryData {
//     #[allow(clippy::useless_format)]
//     /// Creates a format that is just a little more pleasant to the eye
//     #[must_use]
//     pub fn to_printable(&self) -> String {

//         // match self.data_type {
//         //     #[cfg(feature = "font_support")]
//         //     GenericDataType::Font => self.to_font().map_or_else(
//         //         |_| "Not a font.".into(),
//         //         |font| format!("Font: {font:?}"),
//         //     ),
//         //     #[cfg(feature = "image")]
//         //     GenericDataType::Image => format!("Bytes: {:?}", self.to_image()),
//         //     GenericDataType::Audio => format!("Audio: {:?}", "<Unsupported>"),
//         //     GenericDataType::ListOfText => {
//         //         format!("List of text: {{Preview not available}}")
//         //     }
//         //     GenericDataType::Color => self.to_color().map_or_else(
//         //         || "Not a color.".into(),
//         //         |color| {
//         //             format!(
//         //                 "Color: {:?} | r{} g{} b{} a{}",
//         //                 color,
//         //                 color.red(),
//         //                 color.green(),
//         //                 color.blue(),
//         //                 color.alpha()
//         //             )
//         //         },
//         //     ),
//         //     _ => format!("Bytes: {:?}", self.as_bytes()),
//         // }
//     }
// }

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// What type the data is expected to be
///
/// If you don't know what type the data is, use [`Bytes`](Self::Bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub enum GenericDataType {
    #[default]
    /// Raw bytes/custom
    Bytes,
    /// Plain Text/Strings
    Text,
    /// Font
    Font,
    /// Image/Buffer
    Image,
    /// Not supported
    Audio,
    /// As a list of strings/file paths
    ListOfText,
    /// As a color
    Color,
    /// The bytes are in a video format
    Video,
    /// The bytes are in a container format
    Archive,
    /// A rich layout text version
    Document,
    /// A 3D construct of possibly a mesh, textures, and or more
    Model,
    /// The given data is in a value codec, for example Json, yalm, or all the other formats that exist nowadays (not limited to text-only formats)
    ///
    /// Be aware that codec formats may be wrongly interpreted at times, resulting in a false categorization of [`Bytes`](Self::Bytes) or [`Text`](Self::Text)
    Codec,
    /// The bytes are in a format that can be executed. Examples include any portable programming language as well as compiled code: binaries, python code, wasm code, c code
    Executable,
}
impl<T> BinaryData<T> {
    #[must_use]
    /// Constructor to load data from raw bytes
    pub const fn from_bytes(data: Vec<u8>, data_type: T) -> Self {
        Self {
            raw_data: data,
            data_type,
        }
    }
}

impl BinaryData {
    // #[must_use]
    // /// Convert the raw bytes to a Number (assumes data is in a binary format like little-endian)
    // pub fn as_number(&self) -> Result<i64, &'static str> {
    //     if self.raw_data.len() < 8 {
    //         return Err(
    //             "Not enough data",
    //         );
    //     }
    //     let number = i64::from_le_bytes(self.raw_data[0..8].try_into()?);
    //     Ok(number)
    // }
    #[cfg(feature = "font_support")]
    /// Convert the raw bytes to a [`fontdue::Font`] if possible
    ///
    /// # Errors
    /// When not a font it will error
    pub fn to_font(&self) -> Result<fontdue::Font, Box<dyn core::error::Error>> {
        let font =
            fontdue::Font::from_bytes(self.raw_data.clone(), fontdue::FontSettings::default())?;
        Ok(font)
    }
    #[must_use]
    /// Get raw bytes
    pub const fn as_bytes(&self) -> &Vec<u8> {
        &self.raw_data
    }
}
