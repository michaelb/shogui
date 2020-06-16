pub mod emscripten_file;
pub mod shogiban;

fn main() -> Result<(), String> {
    // let's do this!
    shogiban::init()?;

    Ok(())
}
