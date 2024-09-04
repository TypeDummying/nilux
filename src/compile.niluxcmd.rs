
use std::process::Command;
use std::path::Path;
use std::fs;

pub fn compile_niluxcmd() -> Result<(), Box<dyn std::error::Error>> {
    let src_dir = Path::new("src/niluxcmd");
    let build_dir = Path::new("build/niluxcmd");

    // Create build directory if it doesn't exist
    fs::create_dir_all(build_dir)?;

    // Compile each .rs file in the src/niluxcmd directory
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let output_path = build_dir.join(file_name);

            println!("Compiling: {}", file_name);

            let status = Command::new("rustc")
                .arg(path)
                .arg("-o")
                .arg(output_path)
                .status()?;

            if !status.success() {
                return Err(format!("Failed to compile {}", file_name).into());
            }
        }
    }

    println!("Nilux commands compiled successfully");
    Ok(())
}
