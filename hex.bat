xargo clean
xargo build --target thumbv7em-none-eabihf
cd target/thumbv7em-none-eabihf/debug
arm-none-eabi-objcopy -O ihex rust_stm32 rust_stm32.hex
copy rust_stm32.hex ..\..\..\rust_stm32.hex
cd ../../../