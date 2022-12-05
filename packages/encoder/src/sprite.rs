use crate::{arguments::get, utils::get_wavs, Arguments};

use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use std::process::Stdio;

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
        .map(String::as_str)
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
        owned_filter: &mut String::new(),
        owned_output_path: &mut String::new(),
    };

    let builder = create_builder(options);
    if DEBUG {
        println!("builder: {:?}", builder.to_command());
        return;
    }
    let ffmpeg = builder.run().await.unwrap();
    let output = ffmpeg.process.wait_with_output().unwrap();
    println!("[encoder]: {}", output.status);
    assert!(output.status.success(), "[encoder]: encoding failed");
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
        owned_filter,
        owned_output_path,
    } = options;

    let hide_banner = Parameter::Single("hide_banner");
    let nostdin = Parameter::Single("nostdin");
    let overwrite = Parameter::Single("y");
    let sample_rate = Parameter::KeyValue("ar", sample_rate);
    let codec = Parameter::KeyValue("c:a", codec);
    let bitrate = Parameter::KeyValue("b:a", bitrate);

    let num_inputs = input_paths.len();
    let out = "[out]";

    let concat = create_concat_filter(num_inputs);
    let resampler = format!("aresample=resampler={}", resampler);
    owned_filter.push_str(&format!("{},{}{}", concat, resampler, out));

    let filter = Parameter::KeyValue("filter_complex", owned_filter); // [out]
    let map = Parameter::KeyValue("map", out);

    owned_output_path.push_str(&format!("{}/{}.{}", output_directory, sprite, extension));
    let output_file = File::new(owned_output_path);
    println!("[encoder]: {}", output_file.url);

    let mut builder = FfmpegBuilder::new()
        .stderr(Stdio::piped())
        .option(hide_banner)
        .option(nostdin)
        .option(overwrite)
        .output(
            output_file
                .option(filter)
                .option(map)
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

/// with two inputs will look like this: ``-filter_complex`` ``[0:0][1:0]concat=n=2:v=0:a=1``
/// with three inputs: ``-filter_complex`` ``[0:0][1:0][2:0]concat=n=3:v=0:a=1``
fn create_concat_filter(num_inputs: usize) -> String {
    let mut concat_filter = String::new();
    for i in 0..num_inputs {
        concat_filter.push_str(&format!("[{}:0]", i));
    }
    concat_filter.push_str(&format!("concat=n={}:v=0:a=1", num_inputs));
    concat_filter
}

// derive clone
struct CreateBuilderOptions<'a> {
    input_paths: &'a Vec<&'a str>,
    output_directory: &'a str,
    sample_rate: &'a str,
    resampler: &'a str,
    codec: &'a str,
    bitrate: &'a str,
    sprite: &'a str,
    extension: &'a str,
    /// used to store owned strings inside the struct since the builder needs references
    /// otherwise we would have to leak the strings we create in the function
    owned_filter: &'a mut String,
    owned_output_path: &'a mut String,
}
