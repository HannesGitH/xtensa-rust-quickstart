# PARTY BIKE

how to (build rust)[HELP/INSTALL_ESP_RUST.md]
build and run files:
- cargo xbuild --features="xtensa-lx-rt/lx6,xtensa-lx/lx6,esp32-hal"
- cargo espflash --chip esp32 --speed 115200 --features="xtensa-lx-rt/lx6,xtensa-lx/lx6,esp32-hal" COM5