use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use std::{fs, process::Stdio};
mod utils;
mod sprite;
use clap::Parser;
use crate::utils::{absolute_path, read_dir};

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
    #[clap(short, long, default_value = "sprite")] // create a sprite file
    sprite: String,
}

#[tokio::main]
async fn main() {
    let Arguments {
        input_directory,
        output_directory,
        sample_rate,
        resampler,
        codec,
        birate,
        input_extension,
        output_extension,
        sprite,
        ..
    } = Arguments::parse();

    if sprite.len() > 0 {
        sprite::sprite().await;
    }

    let input_directory = absolute_path(input_directory)
        .unwrap()
        .display()
        .to_string();

    let output_directory = absolute_path(output_directory)
        .unwrap()
        .display()
        .to_string();

    match fs::create_dir(&output_directory) {
        Ok(_) => {},
        // we don't care if the directory already exists, thats fine
        Err(_) => {},
    }

    let input_paths = read_dir(&input_directory, "wav");

    let mut output_paths = vec![];
    for path in input_paths.to_owned() {
        let output_path = path.replace(&input_directory, &output_directory);
        let output_path = output_path.replace(&input_extension, &output_extension);
        output_paths.push(output_path.to_owned());
    }

    let it = input_paths.iter().zip(output_paths.iter());

    let mut handles = vec![];

    for (input, output) in it {
        // start a new thread for each process
        let input = input.to_string();
        let output = output.to_string();
        let sample_rate = sample_rate.to_string();
        let resampler = resampler.to_string();
        let codec = codec.to_string();
        let birate = birate.to_string();

        let handle = tokio::spawn(async move {
            let output_path = output.to_owned();
            let input_file = File::new(&input);
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
                        .option(Parameter::KeyValue("ar", &sample_rate))
                        .option(Parameter::KeyValue("resampler", &resampler))
                        .option(Parameter::KeyValue("c:a", &codec))
                        .option(Parameter::KeyValue("b:a", &birate)),
                );
            let ffmpeg = builder.run().await.unwrap();
            ffmpeg.process.wait_with_output().unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

}
