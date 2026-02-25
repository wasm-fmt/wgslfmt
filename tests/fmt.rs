use insta::{Settings, assert_binary_snapshot, glob};
use std::fs;
use std::path::Path;
use wgslfmt::format;

#[test]
fn fmt_snapshot() {
    let base_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/test_data"));
    glob!(base_dir, "**/*.wgsl", |path| {
        let input = fs::read_to_string(path).unwrap();

        let output = format(&input, None).expect("format failed");

        let mut settings = Settings::clone_current();
        settings.set_snapshot_path(path.parent().unwrap());
        settings.remove_snapshot_suffix();
        settings.set_prepend_module_to_snapshot(false);

        settings.bind(|| {
            let snapshot_name = format!("{}.output", path.file_stem().unwrap().to_str().unwrap());
            assert_binary_snapshot!(&snapshot_name, output.into_bytes());
        });
    });
}
