pub mod entry;

#[macro_export]
macro_rules! cli_entry {
    (
        dir = $dir: literal,
        output_file = $output_path: literal,
        after_setup = [$($after_setup: literal),*]
    ) => {
        use yukino::entry::CommandLineEntry;
        use std::collections::HashMap;
        pub fn main() {
            let crate_path = env!("CARGO_MANIFEST_DIR");
            CommandLineEntry::create(
                format!("{}/{}", crate_path, $dir),
                format!("{}/{}", crate_path, $output_path),
                vec![$($after_setup.to_string()),*],
                vec![],
                vec![]
            ).unwrap().process().unwrap();
        }
    }
}
