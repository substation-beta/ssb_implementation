use std::env;

fn main() {
    // Add profile to crate
    println!("cargo:rustc-env=PROFILE={}", env::var("PROFILE").expect("Build profile should be known!"));

    // Embed version information to binary
    #[cfg(windows)]
    {
        // Imports
        use std::path::Path;
        use std::fs;
        use chrono::{Local, Datelike};

        // Path for temporary manifest file
        let file_path = Path::new(&env::var("OUT_DIR").expect("Build output directory should be known!")).join("manifest.rs");
        // Manifest data
        let pkg_name = env!("CARGO_PKG_NAME");
        let pkg_description = env!("CARGO_PKG_DESCRIPTION");
        let major_version = env!("CARGO_PKG_VERSION_MAJOR");
        let minor_version = env!("CARGO_PKG_VERSION_MINOR");
        let patch_version = env!("CARGO_PKG_VERSION_PATCH");
        let version_string = env!("CARGO_PKG_VERSION");
        let authors = env!("CARGO_PKG_AUTHORS");
        let date = Local::today();
        // Write manifest code into file
        fs::write(&file_path, format!(
            r#"// Version informations
            1 VERSIONINFO
            FILEVERSION {},{},{},0
            PRODUCTVERSION {},{},{},0
            BEGIN
                BLOCK "StringFileInfo"
                BEGIN
                    BLOCK "040904E4"    // Language + codepage in hexadecimal (see further down)
                    BEGIN
                        VALUE "CompanyName",      "{}"
                        VALUE "FileDescription",  "{}"
                        VALUE "FileVersion",      "{}"
                        VALUE "InternalName",     "{}"
                        VALUE "LegalCopyright",   "{}, {}"
                        VALUE "OriginalFilename", "{}.dll"
                        VALUE "ProductName",      "{}"
                        VALUE "ProductVersion",   "{}"
                    END
                END
                BLOCK "VarFileInfo"
                BEGIN
                    VALUE "Translation", 0x409, 1252    // English language (0x409) with ANSI codepage (1252)
                END
            END"#,
            major_version, minor_version, patch_version,
            major_version, minor_version, patch_version,
            authors,
            pkg_description,
            version_string,
            pkg_name,
            date.year(), authors,
            pkg_name,
            pkg_name,
            version_string
        )).expect("Couldn't create temporary output file!");
        // Compile and link manifest
        embed_resource::compile(file_path);
    }
}