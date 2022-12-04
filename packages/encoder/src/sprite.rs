use crate::{
    arguments::get,
    utils::{get_wavs, leak_str},
    Arguments,
};

use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use std::process::Stdio;

#[allow(dead_code)]
const DEBUG: bool = false;

pub async fn sprite() {
    let Arguments {
        input_directory,
        output_directory,
        sample_rate,
        resampler,
        codec,
        birate: bitrate,
        output_extension,
        sprite,
        ..
    } = get();

    let input_paths = get_wavs(&input_directory);

    // create Vec<&str> from Vec<String>
    let input_paths = input_paths
        .iter()
        .map(|path| path.as_str())
        .collect::<Vec<&str>>();

    let options = CreateBuilderOptions {
        input_paths: &input_paths,
        output_directory: &output_directory,
        sample_rate: &sample_rate,
        resampler: &resampler,
        codec: &codec,
        bitrate: &bitrate,
        sprite: &sprite,
        extension: &output_extension,
        owned_concat: &mut String::new(),
        owned_resampler: &mut String::new(),
    };
    let builder = create_builder(options);
    // println!("builder: {:?}", builder.to_command());
    let ffmpeg = builder.run().await.unwrap();
    ffmpeg.process.wait_with_output().unwrap();
}

fn create_builder(options: CreateBuilderOptions) -> FfmpegBuilder {
    let CreateBuilderOptions {
        input_paths,
        output_directory,
        sample_rate,
        resampler,
        codec,
        bitrate,
        sprite,
        extension,
        owned_resampler,
        owned_concat,
    } = options;

    let hide_banner = Parameter::Single("hide_banner");
    let nostdin = Parameter::Single("nostdin");
    let overwrite = Parameter::Single("y");
    let sample_rate = Parameter::KeyValue("ar", sample_rate);

    owned_resampler.push_str(&format!("aresample=resampler={}", resampler));
    let resampler = Parameter::KeyValue("af", resampler);
    let codec = Parameter::KeyValue("c", codec);
    let bitrate = Parameter::KeyValue("b", bitrate);

    let num_inputs = input_paths.len();

    owned_concat.push_str(&create_concat_filter(num_inputs));
    let concat = Parameter::KeyValue("filter_complex", owned_concat);

    let map = Parameter::KeyValue("map", "[out]");

    let output_path = leak_str(format!("{}/{}.{}", output_directory, sprite, extension));
    let output_file = File::new(output_path);
    println!("[encoder]: {}", output_file.url);

    let mut builder = FfmpegBuilder::new()
        .stderr(Stdio::piped())
        .option(hide_banner)
        .option(nostdin)
        .option(overwrite)
        .output(
            output_file
                .option(concat)
                .option(map)
                // .option(resampler)
                .option(sample_rate)
                .option(codec)
                .option(bitrate),
        );

    for path in input_paths {
        let file = File::new(path);
        builder = builder.input(file);
    }
    builder
}

// with two inputs will look like this: -filter_complex [0:0][1:0]concat=n=2:v=0:a=1[out]
// with three inputs: -filter_complex [0:0][1:0][2:0]concat=n=3:v=0:a=1[out]
fn create_concat_filter(num_inputs: usize) -> String {
    let mut concat_filter = String::new();
    for i in 0..num_inputs {
        concat_filter.push_str(&format!("[{}:0]", i));
    }
    concat_filter.push_str(&format!("concat=n={}:v=0:a=1[out]", num_inputs));
    concat_filter
}

struct CreateBuilderOptions<'a> {
    input_paths: &'a Vec<&'a str>,
    output_directory: &'a str,
    sample_rate: &'a str,
    resampler: &'a str,
    codec: &'a str,
    bitrate: &'a str,
    sprite: &'a str,
    extension: &'a str,
    // used to store owned strings inside the struct since the builder needs references
    // otherwise we would have to leak the strings we create in the function
    owned_resampler: &'a mut String,
    owned_concat: &'a mut String,
}
