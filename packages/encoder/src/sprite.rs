use crate::utils::{self, read_dir, string_to_static_str};
use clap::Parser;
use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use std::fs;
use std::process::Stdio;

#[allow(dead_code)]
const DEBUG: bool = false;

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

pub async fn sprite() {
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

    let input_directory = utils::absolute_path(input_directory)
        .unwrap()
        .display()
        .to_string();

    let output_directory = utils::absolute_path(output_directory)
        .unwrap()
        .display()
        .to_string();

    match fs::create_dir(&output_directory) {
        Ok(_) => {},
        Err(_) => {},
    }

    let input_paths = read_dir(&input_directory, ".wav");

    let mut output_paths = vec![];
    for path in input_paths.to_owned() {
        let output_path = path.replace(&input_directory, &output_directory);
        let output_path = output_path.replace(&input_extension, &output_extension);
        output_paths.push(output_path.to_owned());
    }

    fn create_builder(
        input_paths: Vec<String>,
        output_directory: String,
        sample_rate: String,
        resampler: String,
        codec: String,
        bitrate: String,
        sprite: String,
        extension: String,
    ) -> FfmpegBuilder<'static> {
        let hide_banner = Parameter::Single("hide_banner");
        let nostdin = Parameter::Single("nostdin");
        let overwrite = Parameter::Single("y");

        let sample_rate = Parameter::KeyValue("ar", string_to_static_str(sample_rate));
        let resampler = Parameter::KeyValue(
            "af",
            string_to_static_str(format!("aresample=resampler={}", resampler)),
        );
        let codec = Parameter::KeyValue("c", string_to_static_str(codec));
        let bitrate = Parameter::KeyValue("b", string_to_static_str(bitrate));

        // with two inputs will look like this: -filter_complex [0:0][1:0]concat=n=2:v=0:a=1[out]
        // with three inputs: -filter_complex [0:0][1:0][2:0]concat=n=3:v=0:a=1[out]
        fn create_concat_filter(num_inputs: usize) -> String {
            let mut concat_filter = String::from("");
            for i in 0..num_inputs {
                concat_filter.push_str(&format!("[{}:0]", i));
            }
            concat_filter.push_str(&format!("concat=n={}:v=0:a=1[out]", num_inputs.to_string()));
            concat_filter
        }

        let num_inputs = input_paths.len();

        let concat = Parameter::KeyValue(
            "filter_complex",
            &string_to_static_str(create_concat_filter(num_inputs)),
        );
        let map = Parameter::KeyValue("map", "[out]");

        let output_path = format!("{}/{}.{}", output_directory, sprite, extension);
        let output_file = File::new(string_to_static_str(output_path));
        println!("[encoder]: {}", output_file.url);

        let a = string_to_static_str(input_paths[0].to_string());
        let b = string_to_static_str(input_paths[1].to_owned().to_string());

        FfmpegBuilder::new()
            .stderr(Stdio::piped())
            .option(hide_banner)
            .option(nostdin)
            .option(overwrite)
            .input(File::new(&a))
            .input(File::new(&b))
            .output(
                output_file
                    .option(concat)
                    .option(map)
                    .option(resampler)
                    .option(sample_rate)
                    .option(codec)
                    .option(bitrate),
            )
    }

    // do the actual encoding
    {
        let builder = create_builder(
            input_paths,
            output_directory.to_string(),
            sample_rate.to_string(),
            resampler.to_string(),
            codec.to_string(),
            birate.to_string(),
            sprite.to_string(),
            output_extension.to_string(),
        );
        let ffmpeg = builder.run().await.unwrap();
        ffmpeg.process.wait_with_output().unwrap();
    }
}
