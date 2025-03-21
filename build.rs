use {
    std::{
        env,
        io,
        fs,
        path::Path,
    },
    winresource::WindowsResource,
};

fn main() -> io::Result<()> {
    // Set Windows icon
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("assets/icon.ico")
            .compile()?;
    }

    // Copy assets folder to output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();
    let assets_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets");
    
    if assets_dir.exists() {
        let target_assets = target_dir.join("assets");
        if target_assets.exists() {
            fs::remove_dir_all(&target_assets)?;
        }
        fs::create_dir_all(&target_assets)?;
        for entry in fs::read_dir(assets_dir)? {
            let entry = entry?;
            let target_path = target_assets.join(entry.file_name());
            fs::copy(entry.path(), target_path)?;
        }
    }

    Ok(())
}