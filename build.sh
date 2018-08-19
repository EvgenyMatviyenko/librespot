ssh vagrant@192.168.33.10 "sudo docker run -v ~/librespot-build:/build librespot-cross cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features alsa-backend"
scp vagrant@192.168.33.10:~/librespot-build/arm-unknown-linux-gnueabihf/release/librespot pi@192.168.1.229:~/Documents/
