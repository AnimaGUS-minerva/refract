use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

pub fn expose_under_target(src: &str, dir: Option<&str>, name: &str) -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let suffix = out_dir
        .rsplit("/target/")
        .collect::<Vec<_>>()[0];
    let target_dir = out_dir.replace(suffix, "");
    if !target_dir.ends_with("/target/") {
        return Err(Error::new(ErrorKind::Other, "Failed to resolve the 'target' dir."));
    }
    println!("@@ target_dir: {}", target_dir);

    let dest_dir = if let Some(dir) = dir {
        let dir_path = format!("{}{}/", target_dir, dir);
        fs::create_dir(&dir_path).unwrap_or_else(|why| {
            println!("@@ fs::create_dir(): {:?}", why.kind());
        });
        dir_path
    } else {
        target_dir
    };
    println!("@@ dest_dir: {}", dest_dir);

    assert!(dest_dir.ends_with("/"));
    let src_out = format!("{}{}", dest_dir, name);

    let src = format!("{}/{}", env::var("CARGO_MANIFEST_DIR").unwrap(), src);
    println!("@@ src: {}\n  ----> src_out: {}", src, src_out);
    fs::copy(src, src_out)?;

    Ok(())
}
