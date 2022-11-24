use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use std::fs;
use std::process::Stdio;
mod path;

fn read_dir(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();

    let mut files = vec![];
    for path in paths {
        let path = path.unwrap().path();
        let absolute_path = path::absolute_path(path).unwrap().display().to_string();

        // if file ends with extension .wav
        if absolute_path.ends_with(".wav") {
            files.push(absolute_path);
        }
    }

    return files;
}
use clap::Parser;

#[derive(Parser, Default, Debug)]
struct Arguments {
    #[clap(short, long, default_value = "audio-sources")]
    input_directory: String,
    #[clap(short, long, default_value = "audio-encoded")]
    output_directory: String,
    #[clap(long, default_value = "wav")]
    input_extension: String,
    #[clap(long, default_value = "webm")]
    output_extension: String,
    #[clap(short, long, default_value = "48000")]
    sample_rate: String,
    #[clap(short, long, default_value = "soxr")] // soxr, swr
    resampler: String,
    #[clap(short, long, default_value = "libopus")] // libopus, libvorbis, libmp3lame, libfdk_aac
    codec: String,
    #[clap(short, long, default_value = "128k")] // libopus, libvorbis, libmp3lame, libfdk_aac
    birate: String,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    let input_directory = path::absolute_path(&args.input_directory)
        .unwrap()
        .display()
        .to_string();

    let output_directory = path::absolute_path(&args.output_directory)
        .unwrap()
        .display()
        .to_string();

    println!("[encoder]: input directory: {}", input_directory);
    println!("[encoder]: output directory: {}", output_directory);

    #[allow(unused_must_use)]
    {
        fs::create_dir(&output_directory);
    }
    fn copy_string(str: String) -> String {
        return str.to_string();
    }

    println!("{:?}", args);
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let input_paths = read_dir(&input_directory);

    let mut output_paths = vec![];
    for path in input_paths.to_owned() {
        let output_path = path.replace(&input_directory, &output_directory);
        let output_path = output_path.replace(&args.input_extension, &args.output_extension);
        output_paths.push(output_path.to_owned());
    }

    let it = input_paths.iter().zip(output_paths.iter());

    for (input, output) in it {
        // default
        let output_path = output.to_owned();
        let input_file = File::new(input);
        let output_file = File::new(&output_path);

        println!("[encoder]: {}", output_file.url);
        let builder = FfmpegBuilder::new()
            .stderr(Stdio::piped())
            .option(Parameter::Single("hide_banner"))
            .option(Parameter::Single("nostdin"))
            .option(Parameter::Single("y"))
            .input(input_file)
            .output(
                output_file
                    .option(Parameter::Single("y"))
                    .option(Parameter::KeyValue("ar", &args.sample_rate))
                    .option(Parameter::KeyValue("c:a", &args.codec))
                    .option(Parameter::KeyValue("b:a", &args.birate)),
            );
        let ffmpeg = builder.run().await.unwrap();
        ffmpeg.process.wait_with_output().unwrap();
    }
}
