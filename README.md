# Mirl Collections (0.0.0-alpha)

#### Mico - A misc lib for specialized containers

<details>
<summary>Flags</summary>

### Default:

**Core**

- ~~`std` (Default)~~ - `std` is required
- `all`
- `c_compatible`

**Codec**

- `all_codecs`
- `serde`
- `bitcode`
- `wincode` (bitcode recommended)
- `zerocopy`
- `compactly`

**Enum**

- `all_enum_extensions`
- `strum`
- `enum_ext`

</details>

### Entry points

- `VecMap` - A HashMap like structure composed using Vec
- `NonEmptyVec` - A Vec type that cannot be empty
- `SparseVec` - A Vec type with an O(1) pushing, deletion, and random access 
- `BinaryData` An plugin extendible `Vec<u8>` wrapper to easily convert binary data into other formats

### Purpose

A collection is specialized/custom maps for unusual situations

### Disclaimer

The `NonEmptyVec` struct will be changed in the future to be more performant

### Origin

I needed the `VecMap` struct in Mirl Values but it didn't fit in there so this crate was created instead and filled further with items from Mirl Core
