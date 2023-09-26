use std::{env, io};

use winres::WindowsResource;

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("assets/rustyed_icon64.ico")
            .compile()?;
    }
    Ok(())
}
