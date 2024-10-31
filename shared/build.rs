use std::path::PathBuf;

use crux_core::typegen::TypeGen;
use crux_http::HttpError;
use vercre_wallet::{App, Aspect};

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../wallet");

    let mut gen = TypeGen::new();
    let out_dir = PathBuf::from("./generated");
    gen.register_app::<App>()?;
    // Shouldn't need to do this, but...
    gen.register_type::<HttpError>()?;

    // Register other types the code generator is having trouble inferring
    gen.register_type::<Aspect>()?;

    gen.swift("SharedTypes", out_dir.join("swift"))?;
    gen.java("io.vercre.wallet.shared_types", out_dir.join("java"))?;
    gen.typescript("shared_types", out_dir.join("typescript"))?;

    Ok(())
}
