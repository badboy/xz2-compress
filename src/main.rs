extern crate walkdir;
extern crate xz2;
extern crate tar;

use std::{env, process};
use std::io::BufWriter;
use std::fs::File;
use std::path::PathBuf;

use walkdir::WalkDir;
use xz2::write::XzEncoder;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: xz2-compress file out");
        process::exit(1);
    }

    let in_directory = args[0].clone();
    let out  = args[1].clone();
    let out = PathBuf::from(out);

    let walk = WalkDir::new(in_directory);

    let outf = File::create(&out).expect("Can't open output file");
    let writer = BufWriter::new(outf);

    let encoder = XzEncoder::new(writer, 6);
    let mut builder = tar::Builder::new(encoder);

    for entry in walk.into_iter().filter_map(|e| e.ok()) {
        let typ = entry.file_type();
        let path = entry.path();

        if path == out {
            println!("Skipping {}", path.display());
        }

        if typ.is_file() {
            println!("Appending file: {}", path.display());
            builder.append_path(path).expect("Can't append file");
        }
    }

    let encoder = builder.into_inner().expect("Creating tar data failed");
    let _writer = encoder.finish().expect("Can't finish writing compressed stream");
    println!("Written compressed stream to {}", out.display());
}
