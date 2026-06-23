use super::{
    super::{BinaryData, GenericDataType},
    BinaryDataPlugin, BinaryDataPluginTrait,
};
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// The plugin struct for handling plain text
pub struct ListOfStringsBinaryDataPlugin;

/// The static position for [`ListOfStringsBinaryDataPlugin`] as to allow for dyn
pub static LIST_OF_STRINGS_BINARY_DATA_PLUGIN: ListOfStringsBinaryDataPlugin =
    ListOfStringsBinaryDataPlugin;

impl BinaryDataPluginTrait<GenericDataType> for ListOfStringsBinaryDataPlugin {
    fn visualize_data(
        &self,
        f: &mut std::fmt::DebugStruct<'_, '_>,
        bin: &BinaryData<GenericDataType>,
    ) -> bool {
        if !matches!(bin.data_type, GenericDataType::ListOfText) {
            return false;
        }
        f.field("raw_data@ListOfStrings", &bin.to_list_of_strings());

        true
    }
}
/// The trait, implemented for [`BinaryData`]
pub trait ListOfStringsFromBinaryDataPluginTrait {
    /// Convert the raw bytes to a &str (if valid UTF-8)
    ///
    /// # Errors
    /// If the data is not in utf8 format
    fn to_list_of_strings(&self) -> Result<Vec<String>, ListOfStringsDecodeError>;
}
/// The trait, implemented for [`BinaryData`]
pub trait ListOfStringsToBinaryDataPluginTrait: Sized {
    /// Create a [`BinaryData`] from a string
    ///
    /// # Errors
    /// [`ListOfStringsEncodeError`]
    fn from_list_of_strings(string: &[&str]) -> Result<Self, ListOfStringsEncodeError>;
}
impl<T> ListOfStringsFromBinaryDataPluginTrait for BinaryData<T> {
    fn to_list_of_strings(&self) -> Result<Vec<String>, ListOfStringsDecodeError> {
        bytes_to_strings(&self.raw_data)
    }
}
impl ListOfStringsToBinaryDataPluginTrait for BinaryData<GenericDataType> {
    fn from_list_of_strings(string: &[&str]) -> Result<Self, ListOfStringsEncodeError> {
        Ok(Self {
            raw_data: strings_to_bytes(string)?,
            data_type: GenericDataType::ListOfText,
        })
    }
}
impl ListOfStringsToBinaryDataPluginTrait for BinaryData<String> {
    fn from_list_of_strings(string: &[&str]) -> Result<Self, ListOfStringsEncodeError> {
        Ok(Self {
            raw_data: strings_to_bytes(string)?,
            data_type: "list_of_string@utf8".to_string(),
        })
    }
}

inventory::submit! {
    BinaryDataPlugin {
        name: "list_of_strings",
        plugin: &LIST_OF_STRINGS_BINARY_DATA_PLUGIN,
    }
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all(zerocopy = false))]
/// Errors that might occur when encoding a list of strings into raw bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ListOfStringsEncodeError {
    /// List has too many strings, exceeding [`u32::MAX`]
    TooManyStrings(usize),
    /// String is too long in bytes, exceeding [`u32::MAX`]
    StringTooLong(usize),
}

impl std::fmt::Display for ListOfStringsEncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyStrings(n) => {
                write!(f, "too many strings: {n} exceeds u32::MAX")
            }
            Self::StringTooLong(n) => {
                write!(f, "string too long: {n} bytes exceeds u32::MAX")
            }
        }
    }
}

impl std::error::Error for ListOfStringsEncodeError {}

#[cfg_attr(
    feature = "mirl_derive",
    mirl_derive::derive_all(serde = false, bitcode = false, zerocopy = false)
)]
/// Errors that might occur when decoding a list of strings into raw bytes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListOfStringsDecodeError {
    /// Unexpected end of data
    UnexpectedEof,
    /// Data is not valid utf8
    InvalidUtf8(std::string::FromUtf8Error),
}

impl std::fmt::Display for ListOfStringsDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEof => write!(f, "unexpected end of input"),
            Self::InvalidUtf8(e) => write!(f, "invalid UTF-8: {e}"),
        }
    }
}

impl std::error::Error for ListOfStringsDecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidUtf8(e) => Some(e),
            Self::UnexpectedEof => None,
        }
    }
}

/// Convert a list of strings into a list of bytes.
///
/// # Errors
/// [`ListOfStringsEncodeError`]
pub fn strings_to_bytes(list: &[&str]) -> Result<Vec<u8>, ListOfStringsEncodeError> {
    let count = u32::try_from(list.len())
        .map_err(|_| ListOfStringsEncodeError::TooManyStrings(list.len()))?;

    let mut bytes = Vec::new();
    bytes.extend_from_slice(&count.to_le_bytes());

    for s in list {
        let string_bytes = s.as_bytes();
        let len = u32::try_from(string_bytes.len())
            .map_err(|_| ListOfStringsEncodeError::StringTooLong(string_bytes.len()))?;
        bytes.extend_from_slice(&len.to_le_bytes());
        bytes.extend_from_slice(string_bytes);
    }

    Ok(bytes)
}

/// Convert a list of bytes into a list of strings.
///
/// # Errors
/// [`ListOfStringsDecodeError`]
pub fn bytes_to_strings(list: &[u8]) -> Result<Vec<String>, ListOfStringsDecodeError> {
    fn read_u32(buf: &[u8], cursor: &mut usize) -> Result<u32, ListOfStringsDecodeError> {
        let bytes = buf
            .get(*cursor..*cursor + 4)
            .ok_or(ListOfStringsDecodeError::UnexpectedEof)?;
        *cursor += 4;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    let mut cursor = 0;

    let num_strings = read_u32(list, &mut cursor)? as usize;
    let mut strings = Vec::with_capacity(num_strings);

    for _ in 0..num_strings {
        let len = read_u32(list, &mut cursor)? as usize;
        let bytes = list
            .get(cursor..cursor + len)
            .ok_or(ListOfStringsDecodeError::UnexpectedEof)?;
        strings.push(
            String::from_utf8(bytes.to_vec()).map_err(ListOfStringsDecodeError::InvalidUtf8)?,
        );
        cursor += len;
    }

    Ok(strings)
}
