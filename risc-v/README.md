# Environment configuration

`sudo apt-get update`
`sudo apt-get install qemu-system-misc`
`rustup target add riscv64imac-unknown-none-elf`
`rustup default stable`

## debug instalation

```
git clone https://github.com/riscv/riscv-binutils-gdb.git
cd riscv-binutils-gdb
./configure --target=riscv64-unknown-elf --prefix=/opt/riscv
make
sudo make install
```
Adicionar o path

export PATH=/opt/riscv/bin:$PATH

```
sudo apt-get install build-essential bison flex libgmp3-dev libmpfr-dev libmpc-dev texinfo
git clone https://github.com/riscv/riscv-gnu-toolchain
cd riscv-gnu-toolchain
./configure --prefix=/opt/riscv --with-arch=rv64gc --with-abi=lp64d
make
```
configure the PATH
`riscv64-unknown-elf-gdb --version`

## Execute the code
To execute, run `cargo build --release` and then

```
qemu-system-riscv64 \
    -nographic \
    -machine virt \
    -kernel target/riscv64imac-unknown-none-elf/release/app \
    -bios none
```

### riscv-vga-sample
https://github.com/neri/riscv-vga-sample
