use svgcleaner::{self};

pub mod defaults;

pub fn minify_svg(svg_text: &str) -> Result<String, String> {
    let doc = svgdom::Document::from_str_with_opt(svg_text, &defaults::parse_options()).ok();

    match doc {
        Some(mut doc) => {
            let _ = svgcleaner::cleaner::clean_doc(
                &mut doc,
                &defaults::cleaning_options(),
                &defaults::write_options(),
            );
            let mut buffer: Vec<u8> = vec![];
            svgcleaner::cleaner::write_buffer(&doc, &defaults::write_options(), &mut buffer);

            let string = String::from_utf8(buffer).ok();

            string.ok_or_else(|| "Unable save as UTF8".to_string())
        }

        None => Err("Unable to parse svg".to_string()),
    }
}
