use clap::Parser;
#[derive(Parser, Default, Debug)]
pub struct Arguments {
    #[clap(short, long, default_value = "audio-sources")]
    pub input_directory: String,
    #[clap(short, long, default_value = "audio-encoded")]
    pub output_directory: String,
    #[clap(long, default_value = "wav")]
    pub input_extension: String,
    #[clap(long, default_value = "webm")]
    pub output_extension: String,
    #[clap(short, long, default_value = "48000")]
    pub sample_rate: String,
    #[clap(short, long, default_value = "soxr")] // soxr, swr
    pub resampler: String,
    #[clap(short, long, default_value = "libopus")] // libopus, libvorbis, libmp3lame, libfdk_aac
    pub codec: String,
    #[clap(short, long, default_value = "128k")] // libopus, libvorbis, libmp3lame, libfdk_aac
    pub birate: String,
    #[clap(short, long, default_value = "sprite")] // create a sprite file
    pub sprite: String,
}

pub fn get() -> Arguments {
    Arguments::parse()
}
