use cargo_gpu_install::install::Install;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read when the script runs, not baked in when it was compiled: cargo does not compile a build
    // script again when the project's folder moves, and a baked path then names the old folder
    let shaders = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("../shaders");
    let backend = Install::from_shader_crate(shaders.clone()).within_build_script().run()?;
    let mut builder = backend.to_spirv_builder(shaders, "spirv-unknown-vulkan1.2");
    builder.build_script.defaults = true;
    let built = builder.build()?;
    println!("cargo::rustc-env=SHADERS_SPV={}", built.module.unwrap_single().display());
    Ok(())
}
