rm images/*
cargo run --release -- "$@"
ffmpeg -r 60 -y -i images/out%d.png test.mp4