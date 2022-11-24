# encoder
A simple audio encoder

# run

# ffmpeg
You need to have a working installation of ffmpeg in your path.

MacOS
* brew install ffmpeg
Windows
* https://phoenixnap.com/kb/ffmpeg-windows

To make sure you have ffmpeg in your path
type `ffmpeg` in your you command line tool: bash / terminal / zsh


## Example Usage
cargo run --package encoder --release -- -i audio -o dist/audio -b 96k
