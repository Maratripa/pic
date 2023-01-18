use std::{
    fs::File,
    io::{Error, Write},
    path::PathBuf,
};

pub fn move_cursor(stdout: &mut impl Write, x: u32, y: u32) -> Result<(), std::io::Error> {
    stdout.write(b"\x1b[s")?;

    let binding = format!("\x1b[{}:{}H", y + 1, x + 1);
    let buf = binding.as_bytes();
    stdout.write(buf)?;
    stdout.flush()
}

pub fn restore_cursor(stdout: &mut impl Write) -> Result<(), std::io::Error> {
    stdout.write(b"\x1b[u")?;
    stdout.flush()
}

pub fn get_term_size() -> (u32, u32, u32, u32) {
    // TODO: find a way to make that safe
    unsafe {
        let mut ws = libc::winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        libc::ioctl(0, libc::TIOCGWINSZ, &mut ws);
        (
            ws.ws_row as u32,
            ws.ws_col as u32,
            ws.ws_xpixel as u32,
            ws.ws_ypixel as u32,
        )
    }
}

pub fn get_cell_size() -> (u32, u32) {
    let (rows, cols, xpixel, ypixel) = get_term_size();
    return (xpixel / cols, ypixel / rows);
}

pub fn fit_dimensions(width: u32, height: u32, cols: Option<u32>, rows: Option<u32>) -> (u32, u32) {
    let term_size = get_term_size();
    let (col_size, row_size) = match get_cell_size() {
        (0, 0) => (10, 20),
        (c, r) => (c, r),
    };
    let (cols, rows) = match (cols, rows) {
        (None, None) => (term_size.1, term_size.0),
        (Some(c), None) => (c, term_size.0),
        (None, Some(r)) => (term_size.1, r),
        (Some(c), Some(r)) => (c, r),
    };
    let (bound_width, bound_height) = (cols * col_size, rows * row_size);

    if width <= bound_width && height <= bound_height {
        return (width / col_size, height / row_size);
    }

    let w_ratio = width * bound_height;
    let h_ratio = bound_width * height;

    if w_ratio >= h_ratio {
        (cols, (height * bound_width) / (width * row_size))
    } else {
        ((width * bound_height) / (height * col_size), rows)
    }
}

pub fn get_temp_file(prefix: &str) -> Result<(File, PathBuf), std::io::Error> {
    let (tempfile, pathbuf) = tempfile::Builder::new()
        .prefix(prefix)
        .tempfile_in("/tmp/")?
        .keep()?;

    Ok((tempfile, pathbuf))
}

pub fn save_in_tmp_file(buffer: &[u8], file: &mut File) -> Result<(), Error> {
    file.write_all(buffer)?;
    file.flush()
}