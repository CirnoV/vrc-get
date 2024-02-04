use std::fmt::{Debug, Formatter};
use crate::bson::ObjectId;
use crate::lowlevel::FFISlice;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct ProjectType(u32);

impl ProjectType {
    const UNKNOWN: Self = Self(0);
    const LEGACY_SDK2: Self = Self(1);
    const LEGACY_WORLDS: Self = Self(2);
    const LEGACY_AVATARS: Self = Self(3);
    const UPM_WORLDS: Self = Self(4);
    const UPM_AVATARS: Self = Self(5);
    const UPM_STARTER: Self = Self(6);
    const WORLDS: Self = Self(7);
    const AVATARS: Self = Self(8);
    const VPM_STARTER: Self = Self(9);
}

impl Debug for ProjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProjectType::UNKNOWN => f.write_str("Unknown"),
            ProjectType::LEGACY_SDK2 => f.write_str("Legacy SDK2"),
            ProjectType::LEGACY_WORLDS => f.write_str("Legacy Worlds"),
            ProjectType::LEGACY_AVATARS => f.write_str("Legacy Avatars"),
            ProjectType::UPM_WORLDS => f.write_str("UPM Worlds"),
            ProjectType::UPM_AVATARS => f.write_str("UPM Avatars"),
            ProjectType::UPM_STARTER => f.write_str("UPM Starter"),
            ProjectType::WORLDS => f.write_str("Worlds"),
            ProjectType::AVATARS => f.write_str("Avatars"),
            ProjectType::VPM_STARTER => f.write_str("VPM Starter"),
            _ => f.write_fmt(format_args!("Unexpected({})", self.0)),
        }
    }
}

/// Represents a VCC Project
#[derive(Debug)]
pub struct Project {
    path: Box<str>,
    unity_version: Option<Box<str>>,
    created_at: u64, // milliseconds since Unix epoch in UTC
    updated_at: u64, // milliseconds since Unix epoch in UTC
    type_: ProjectType,
    id: ObjectId,
    favorite: bool,
}

#[repr(C)]
pub(crate) struct ProjectFFI {
    path: FFISlice,
    unity_version: FFISlice,
    created_at: u64, // milliseconds since Unix epoch in UTC
    updated_at: u64, // milliseconds since Unix epoch in UTC
    type_: ProjectType,
    id: ObjectId,
    favorite: u8,
}

impl Project {
    pub unsafe fn from_ffi(ffi: ProjectFFI) -> Self {
        Self {
            path: unsafe {
                std::str::from_boxed_utf8_unchecked(FFISlice::as_boxed_byte_slice(ffi.path))
            },
            unity_version: unsafe {
                FFISlice::as_boxed_byte_slice_option(ffi.unity_version)
                    .map(|x| std::str::from_boxed_utf8_unchecked(x))
            },
            created_at: ffi.created_at,
            updated_at: ffi.updated_at,
            type_: ffi.type_,
            id: ffi.id,
            favorite: ffi.favorite != 0,
        }
    }
}
