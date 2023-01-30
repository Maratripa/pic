use crate::support::Protocol;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Options {
    /// Image to preview
    pub path: PathBuf,

    /// x position (0 is left)
    #[arg(short, long)]
    pub x: Option<u32>,
    /// y position (0 is top)
    #[arg(short, long)]
    pub y: Option<u32>,
    /// Number of cols to fit the preview in
    #[arg(short, long)]
    pub cols: Option<u32>,
    /// Number of rows to fit the preview in
    #[arg(short, long)]
    pub rows: Option<u32>,
    /// Upscale image if needed
    #[arg(short, long)]
    pub upscale: bool,
    /// Only show first frame of GIFs
    #[arg(short = 's', long = "static")]
    pub gif_static: bool,
    /// Loop GIFs infinitely
    #[arg(short = 'l', long = "loop")]
    pub gif_loop: bool,
    /// Previewing protocol to use
    #[arg(short, long)]
    pub protocol: Option<Protocol>,
    /// Load image with the given id (kitty only)
    #[arg(long, value_name = "ID")]
    pub load: Option<u32>,
    /// Display image with the given id (kitty only)
    #[arg(long, value_name = "ID")]
    pub display: Option<u32>,
    /// Clear image with the given id (0 for all) (kitty only)
    #[arg(long, value_name = "ID")]
    pub clear: Option<u32>,

    /// Rotate clockwise [90, 180, 270] (blocks and kitty only)
    #[arg(long = "rot", value_parser = valid_rotation_angle, value_name = "VALUE")]
    pub rotation: Option<u32>,
}

const VALID_DEG: &[u32] = &[90, 180, 270];

fn valid_rotation_angle(s: &str) -> Result<u32, String> {
    let deg: u32 = s.parse().map_err(|_| format!("`{s}` isn't a number"))?;
    if VALID_DEG.contains(&deg) {
        Ok(deg)
    } else {
        Err(format!("Rotation value not in {VALID_DEG:?}"))
    }
}
