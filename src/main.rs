use std::env;
use std::io::BufReader;
use std::path::Path;
use std::vec::Vec;

fn main() {
    let valid_paths = parce_args().unwrap_or_else(|error| {
        eprintln!("ERROR: could not parse arguments: {error}");
        std::process::exit(1);
    });

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap_or_else(|error| {
        eprintln!("ERROR: could not create stream: {error}");
        std::process::exit(1);
    });
    let sink = rodio::Sink::try_new(&handle).unwrap_or_else(|error| {
        eprintln!("ERROR: could not create sink: {error}");
        std::process::exit(1);
    });

    for path in &valid_paths {
        sink.append(
            rodio::Decoder::new(BufReader::new(std::fs::File::open(path).unwrap_or_else(
                |error| {
                    eprintln!("ERROR: could not open file: {error}");
                    std::process::exit(1);
                },
            )))
            .unwrap_or_else(|error| {
                eprintln!("ERROR: could not decode file: {error}");
                std::process::exit(1);
            }),
        );
    }
    sink.pause();
    println!("INFO: Playing {} files", sink.len());
    for path in &valid_paths {
        println!("INFO: {}", path);
    }

    sink.play();
    sink.set_volume(0.1);
    loop {
        
    }
}

fn parce_args() -> Result<Vec<String>, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err("Usage: <program> <filename>".into());
    }

    let valid_extensions = vec!["mp3", "wav", "flac", "ogg"];
    let valid_paths: Vec<_> = args
        .into_iter()
        .filter(|path| {
            Path::new(path)
                .extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |ext| valid_extensions.contains(&ext))
        })
        .collect();

    if valid_paths.is_empty() {
        return Err("No valid files found".into());
    }

    if valid_paths.is_empty() {
        Err("No valid files found".into())
    } else {
        Ok(valid_paths)
    }
}
