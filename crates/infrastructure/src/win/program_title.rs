use std::ffi::c_void;
use std::iter::once;

use domain::program::Program;
use windows::Win32::Storage::FileSystem::{
    GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
};
use windows::core::PCWSTR;

use crate::exe_name::exe_name;

/// English (US) + Unicode, used when the file declares no translation of its own.
const FALLBACK_TRANSLATION: &str = "040904b0";

pub fn describe(path: &str) -> Program {
    Program {
        exe: exe_name(path),
        title: file_description(path),
    }
}

fn file_description(path: &str) -> Option<String> {
    let path = wide(path);
    let size = unsafe { GetFileVersionInfoSizeW(PCWSTR(path.as_ptr()), None) };
    if size == 0 {
        return None;
    }
    let mut block = vec![0u8; size as usize];
    unsafe { GetFileVersionInfoW(PCWSTR(path.as_ptr()), None, size, block.as_mut_ptr().cast()) }
        .ok()?;
    let translation = translation(&block).unwrap_or_else(|| FALLBACK_TRANSLATION.to_owned());
    let (text, len) = query(
        &block,
        &format!(r"\StringFileInfo\{translation}\FileDescription"),
    )?;
    let units = unsafe { std::slice::from_raw_parts(text.cast::<u16>(), len as usize) };
    let description = String::from_utf16_lossy(units);
    let description = description.trim_end_matches('\0').trim();
    (!description.is_empty()).then(|| description.to_owned())
}

fn translation(block: &[u8]) -> Option<String> {
    let (pair, len) = query(block, r"\VarFileInfo\Translation")?;
    if len < 4 {
        return None;
    }
    let pair = pair.cast::<u16>();
    let (language, codepage) = unsafe { (pair.read_unaligned(), pair.add(1).read_unaligned()) };
    Some(format!("{language:04x}{codepage:04x}"))
}

fn query(block: &[u8], sub_block: &str) -> Option<(*const c_void, u32)> {
    let sub_block = wide(sub_block);
    let mut value = std::ptr::null_mut();
    let mut len = 0u32;
    let found = unsafe {
        VerQueryValueW(
            block.as_ptr().cast(),
            PCWSTR(sub_block.as_ptr()),
            &mut value,
            &mut len,
        )
    };
    (found.as_bool() && !value.is_null() && len > 0).then_some((value.cast_const(), len))
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(once(0)).collect()
}
