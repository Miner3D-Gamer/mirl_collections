# Mirl Collections (0.0.0-alpha)

#### Mico - A misc lib for specialized containers

<details>
<summary>Flags</summary>

### Default:

**Core**

- ~~`std` (Default)~~ - `std` is required

- `c_compatible`

**Codec**

- `serde`
- `bitcode`
- `wincode` (bitcode recommended)

~~**Enum**~~ - No enums present

- ~~`strum`~~
- ~~`enum_ext`~~

</details>

### Entry points

- `VecMap` struct

### Purpose

A collection is specialized/custom maps for unusual situations

### Disclaimer

This crate currently only features `VecMap`, soon other containers will be ported over from mirl_core/mirl

### Origin

I needed the `VecMap` struct in Mirl Values but it didn't fit in there
