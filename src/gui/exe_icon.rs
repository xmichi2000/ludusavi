//! Reading the icon out of a program, for games that have no cover art.

#![cfg_attr(not(target_os = "windows"), allow(unused))]

use crate::path::StrictPath;

/// Icons are square, and this is a common size to find inside a program.
const SIZE: u32 = 32;

/// Read the icon of a program as RGBA pixels, `SIZE` by `SIZE`.
#[cfg(target_os = "windows")]
fn read_rgba(executable: &StrictPath) -> Option<Vec<u8>> {
    use windows::{
        Win32::Graphics::Gdi::{
            BI_RGB, BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, DeleteObject, GetDC, GetDIBits, GetObjectW,
            HGDIOBJ, ReleaseDC,
        },
        Win32::UI::Shell::ExtractIconExW,
        Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO},
        core::PCWSTR,
    };

    let path: Vec<u16> = executable
        .render()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();

    unsafe {
        let mut icon = HICON::default();
        // A program may have no icon at all, in which case nothing is written.
        let extracted = ExtractIconExW(PCWSTR(path.as_ptr()), 0, Some(&mut icon), None, 1);
        if extracted == 0 || icon.is_invalid() {
            return None;
        }

        let mut info = ICONINFO::default();
        let result = (|| {
            GetIconInfo(icon, &mut info).ok()?;

            let mut bitmap = BITMAP::default();
            let written = GetObjectW(
                HGDIOBJ(info.hbmColor.0),
                std::mem::size_of::<BITMAP>() as i32,
                Some(&mut bitmap as *mut _ as *mut _),
            );
            if written == 0 || bitmap.bmWidth <= 0 || bitmap.bmHeight <= 0 {
                return None;
            }

            let width = bitmap.bmWidth;
            let height = bitmap.bmHeight;

            let mut header = BITMAPINFO::default();
            header.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
            header.bmiHeader.biWidth = width;
            // Negative height gives us the rows from top to bottom.
            header.bmiHeader.biHeight = -height;
            header.bmiHeader.biPlanes = 1;
            header.bmiHeader.biBitCount = 32;
            header.bmiHeader.biCompression = BI_RGB.0;

            let mut buffer = vec![0u8; (width * height * 4) as usize];
            let dc = GetDC(None);
            let copied = GetDIBits(
                dc,
                info.hbmColor,
                0,
                height as u32,
                Some(buffer.as_mut_ptr() as *mut _),
                &mut header,
                DIB_RGB_COLORS,
            );
            ReleaseDC(None, dc);
            if copied == 0 {
                return None;
            }

            // Windows gives us blue/green/red/alpha, but images want red/green/blue/alpha.
            for pixel in buffer.chunks_exact_mut(4) {
                pixel.swap(0, 2);
            }

            image::RgbaImage::from_raw(width as u32, height as u32, buffer).map(|image| {
                image::imageops::resize(&image, SIZE, SIZE, image::imageops::FilterType::Lanczos3).into_raw()
            })
        })();

        if !info.hbmColor.is_invalid() {
            let _ = DeleteObject(HGDIOBJ(info.hbmColor.0));
        }
        if !info.hbmMask.is_invalid() {
            let _ = DeleteObject(HGDIOBJ(info.hbmMask.0));
        }
        let _ = DestroyIcon(icon);

        result
    }
}

#[cfg(not(target_os = "windows"))]
fn read_rgba(_executable: &StrictPath) -> Option<Vec<u8>> {
    None
}

/// Save the icon of a program as a PNG file, so that it can be shown like other cover art.
pub fn save_as_png(executable: &StrictPath, target: &StrictPath) -> Option<()> {
    let rgba = read_rgba(executable)?;
    let image = image::RgbaImage::from_raw(SIZE, SIZE, rgba)?;

    target.parent()?.create_dirs().ok()?;
    image
        .save_with_format(target.as_std_path_buf().ok()?, image::ImageFormat::Png)
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn can_read_the_icon_of_a_program() {
        // Windows programs reliably have an icon.
        let rgba = read_rgba(&StrictPath::new("C:/Windows/System32/notepad.exe"));

        let rgba = rgba.expect("no icon was read");
        assert_eq!((SIZE * SIZE * 4) as usize, rgba.len());
        // A blank icon would be suspicious.
        assert!(rgba.iter().any(|x| *x != 0));
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn can_save_an_icon_as_an_image() {
        let target = StrictPath::new(format!("{}/tmp/exe-icon-test.png", crate::prelude::app_dir().render()));
        let _ = target.remove();

        save_as_png(&StrictPath::new("C:/Windows/System32/notepad.exe"), &target).expect("no icon was saved");

        assert!(target.is_file());
        let image = image::open(target.as_std_path_buf().unwrap()).expect("the file is not an image");
        assert_eq!((SIZE, SIZE), (image.width(), image.height()));

        let _ = target.remove();
    }

    #[test]
    fn ignores_a_file_that_is_not_a_program() {
        assert_eq!(None, read_rgba(&StrictPath::new("C:/nonexistent-program.exe")));
    }
}
