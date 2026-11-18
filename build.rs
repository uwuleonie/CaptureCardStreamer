// Builds the app icon from assets/app-icon.png (the seekclan flower pot):
// crops away the transparent border so the pot fills the icon, writes a
// 256 px PNG (window/taskbar icon, embedded via include_bytes!) and a
// multi-resolution .ico, and embeds the .ico into the exe on Windows.

use std::env;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use image::imageops::FilterType;
use image::RgbaImage;

const BASE: u32 = 256;
const SIZES: &[u32] = &[16, 24, 32, 48, 64, 128, 256];
const SOURCE: &str = "assets/app-icon.png";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={SOURCE}");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let png_256 = out_dir.join("icon-256.png");
    let ico_path = out_dir.join("icon.ico");

    let square = load_square_icon(&manifest_dir.join(SOURCE));
    let base = scale(&square, BASE);
    base.save(&png_256).expect("write icon-256.png");
    write_ico(&square, &ico_path);

    // Check the TARGET os, not the host os: a build script is compiled for
    // the host, so #[cfg(target_os)] here would describe the build machine.
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        embed_windows_icon(&ico_path);
    }
}

/// Load the source PNG, crop it to the bounding box of its non-transparent
/// pixels and centre that on a square transparent canvas with a small margin.
fn load_square_icon(path: &Path) -> RgbaImage {
    let img = image::open(path)
        .unwrap_or_else(|e| panic!("could not open {}: {e}", path.display()))
        .to_rgba8();
    let (w, h) = img.dimensions();
    let (mut x0, mut y0, mut x1, mut y1) = (w, h, 0u32, 0u32);
    for (x, y, p) in img.enumerate_pixels() {
        if p[3] > 0 {
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
    }
    if x1 < x0 || y1 < y0 {
        // Fully transparent image: nothing to crop.
        return img;
    }
    let cw = x1 - x0 + 1;
    let ch = y1 - y0 + 1;
    let side = cw.max(ch);
    let pad = side / 16;
    let canvas = side + 2 * pad;
    let cropped = image::imageops::crop_imm(&img, x0, y0, cw, ch).to_image();
    let mut out = RgbaImage::new(canvas, canvas);
    image::imageops::overlay(
        &mut out,
        &cropped,
        ((canvas - cw) / 2) as i64,
        ((canvas - ch) / 2) as i64,
    );
    out
}

/// Pixel-art friendly scaling: nearest neighbour when enlarging keeps the
/// pixels crisp, Lanczos when shrinking keeps small sizes readable.
fn scale(img: &RgbaImage, size: u32) -> RgbaImage {
    let filter = if size >= img.width() {
        FilterType::Nearest
    } else {
        FilterType::Lanczos3
    };
    image::imageops::resize(img, size, size, filter)
}

fn write_ico(square: &RgbaImage, path: &Path) {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    for &sz in SIZES {
        let resized = scale(square, sz);
        let mut png_buf = Vec::new();
        image::DynamicImage::ImageRgba8(resized)
            .write_to(&mut Cursor::new(&mut png_buf), image::ImageFormat::Png)
            .expect("encode png");
        let entry = ico::IconImage::read_png(&png_buf[..]).expect("decode png for ico");
        let dir_entry = ico::IconDirEntry::encode(&entry).expect("ico encode");
        icon_dir.add_entry(dir_entry);
    }
    let mut file = fs::File::create(path).expect("create icon.ico");
    icon_dir.write(&mut file).expect("write icon.ico");
}

/// Embed icon.ico (from OUT_DIR) as the exe icon. winresource locates the
/// resource compiler itself: rc.exe from the Windows SDK on MSVC, windres
/// on GNU (from PATH, overridable via the WINDRES env var). A failure only
/// costs the icon, never the build.
fn embed_windows_icon(ico_path: &Path) {
    let Some(ico) = ico_path.to_str() else {
        println!("cargo:warning=icon path is not valid UTF-8, exe will have no icon");
        return;
    };
    let mut res = winresource::WindowsResource::new();
    if let Ok(windres) = env::var("WINDRES") {
        res.set_windres_path(&windres);
    }
    res.set_icon(ico);
    res.set("ProductName", "CaptureCardStreamer");
    res.set("FileDescription", "CaptureCardStreamer");
    if let Err(e) = res.compile() {
        println!("cargo:warning=could not embed exe icon: {e}");
    }
}
