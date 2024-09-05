use std::{io, fs};
use std::path::Path;

fn read_svg_dir(dir: &Path) -> io::Result<Vec<String>> {
    let mut svg_array: Vec<String> = vec![];
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        svg_array.push(path
            .to_str()
            .unwrap()
            .to_string());
    }
    return Ok(svg_array);
}

pub fn safe_read_svg_dir(dir: &Path) -> Vec<String> {
    let anything = read_svg_dir(dir);
    let svg_array = match anything {
        Ok(svg) => svg,
        Err(_) => vec![]
    };
    return svg_array;
}

