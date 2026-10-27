// This reference compiles the guest's exact tracing source for the host CPU.
#[path = "../../src/tracer.rs"]
mod tracer;

use std::{env, fs::{self, File}, io::{BufWriter, Write}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directory = env::args().nth(1).ok_or("expected evidence directory")?;
    let directory = Path::new(&directory);
    fs::create_dir_all(directory)?;
    unsafe {
        tracer::postcard();
        let mut output = BufWriter::new(File::create(directory.join("native.linear.f32"))?);
        for pixel in &*core::ptr::addr_of!(tracer::PIXELS) {
            for channel in pixel {
                output.write_all(&channel.to_le_bytes())?;
            }
        }
        output.flush()?;
        fs::write(directory.join("native.surfaces"), &*core::ptr::addr_of!(tracer::SURFACES))?;
        fs::write(directory.join("native.reflections"), &*core::ptr::addr_of!(tracer::REFLECTIONS))?;
    }
    Ok(())
}
