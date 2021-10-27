pub mod entry;

#[macro_export]
macro_rules! cli_entry {
    (
        dir = $dir: literal,
        output_file = $output_path: literal
    ) => {
        use std::collections::HashMap;
        use yukino::entry::CommandLineEntry;
        pub fn main() {
            let crate_path = env!("CARGO_MANIFEST_DIR");
            CommandLineEntry::create(
                format!("{}/{}", crate_path, $dir),
                format!("{}/{}", crate_path, $output_path),
                vec![],
                vec![],
            )
            .unwrap()
            .process()
            .unwrap();
        }
    };
}
