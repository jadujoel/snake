use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use std::process::Stdio;
use utils::{create_dir, get_absolute, get_wavs};
mod arguments;
mod sprite;
mod utils;
use arguments::{get, Arguments};

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
    } = get();

    let input_directory = get_absolute(&input_directory);
    let output_directory = get_absolute(&output_directory);
    let input_paths = get_wavs(&input_directory);

    create_dir(&output_directory);

    let mut output_paths = vec![];
    for path in input_paths.clone() {
        let output_path = path.replace(&input_directory, &output_directory);
        let output_path = output_path.replace(&input_extension, &output_extension);
        output_paths.push(output_path.clone());
    }

    let mut handles = vec![];
    let it = input_paths.iter().zip(output_paths.iter());
    for (input, output) in it {
        // start a new thread for each process
        let input = input.to_string();
        let output = output.to_string();
        let sample_rate = sample_rate.to_string();
        let resampler = resampler.to_string();
        let codec = codec.to_string();
        let birate = birate.to_string();

        let handle = tokio::spawn(async move {
            let output_path = output.clone();
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
                        .option(Parameter::KeyValue("c", &codec))
                        .option(Parameter::KeyValue("b", &birate)),
                );
            let ffmpeg = builder.run().await.unwrap();
            ffmpeg.process.wait_with_output().unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    if !sprite.is_empty() {
        sprite::sprite().await;
    }
}
