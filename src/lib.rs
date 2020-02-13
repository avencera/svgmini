use svgcleaner::{self, CleaningOptions};
use svgdom::{Indent, ParseOptions, WriteOptions};

pub mod defaults;

fn cleaning_options() -> CleaningOptions {
    CleaningOptions {
        remove_unused_defs: true,
        convert_shapes: true,
        remove_title: true,
        remove_desc: true,
        remove_metadata: true,
        remove_dupl_linear_gradients: true,
        remove_dupl_radial_gradients: true,
        remove_dupl_fe_gaussian_blur: true,
        ungroup_groups: true,
        ungroup_defs: true,
        group_by_style: true,
        merge_gradients: true,
        regroup_gradient_stops: true,
        remove_invalid_stops: true,
        remove_invisible_elements: true,
        resolve_use: true,
        remove_version: true,
        remove_unreferenced_ids: true,
        trim_ids: true,
        remove_text_attributes: true,
        remove_unused_coordinates: true,
        remove_default_attributes: true,
        remove_xmlns_xlink_attribute: true,
        remove_needless_attributes: true,
        remove_gradient_attributes: true,
        apply_transform_to_gradients: true,
        apply_transform_to_shapes: true,
        paths_to_relative: true,
        remove_unused_segments: true,
        convert_segments: true,
        apply_transform_to_paths: false,
        coordinates_precision: 6,
        properties_precision: 6,
        paths_coordinates_precision: 8,
        transforms_precision: 8,
        ..Default::default()
    }
}

fn write_options() -> WriteOptions {
    WriteOptions {
        indent: Indent::None,
        join_arc_to_flags: true,
        use_compact_path_notation: true,
        attributes_indent: Indent::None,
        remove_leading_zero: true,
        use_single_quote: false,
        remove_duplicated_path_commands: true,
        use_implicit_lineto_commands: true,
        trim_hex_colors: true,
        ..Default::default()
    }
}

pub fn minify_svg(svg_text: &str) -> Result<String, String> {
    let doc = svgdom::Document::from_str_with_opt(
        svg_text,
        &ParseOptions {
            parse_comments: false,
            skip_unresolved_classes: false,
            ..Default::default()
        },
    )
    .ok();

    match doc {
        Some(mut doc) => {
            let _ = svgcleaner::cleaner::clean_doc(&mut doc, &cleaning_options(), &write_options());
            let mut buffer: Vec<u8> = vec![];
            svgcleaner::cleaner::write_buffer(&doc, &write_options(), &mut buffer);

            let string = String::from_utf8(buffer).ok();

            string.ok_or_else(|| "Unable save as UTF8".to_string())
        }

        None => Err("Unable to parse svg".to_string()),
    }
}
