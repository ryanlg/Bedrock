# Project Redstone
- It's (supposedly in the end) a something that looks somewhat like a kernel.

## Specs
- Based on `x86_64-elf` arch.

## Requirements
- QEMU
- Homebrew
- LLVM Suite (LLD, more specifically)
- GRUB (grub-mkresure with `x86_64-elf` as target, more specifically)

## Setup
- Since my development machine is a Mac, this guide is tailored specifically for macOS.

### Build LLVM
- There's many tutorials on this. You can follow [this](https://llvm.org/docs/GettingStarted.html) if you are lazy like me.
- The compilation and installation should be pretty straight forward with no strange hiccups, better than GNU with their delicate ecosystem.

### Build GRUB
1. Define your `target` and `prefix` in env var
```
export PREFIX="/usr/local/opt"
export TARGET=x86_64-pc-elf
export PATH="$PREFIX/binutils/bin:$PREFIX/gcc/bin:$PREFIX/grub/bin:$PATH"
```

#### Build `Binutils`
1. Download `binutils` tarball
2. Build
```
mkdir build-binutils && cd build-binutils
../binutils/configure --target=$TARGET --prefix=$PREFIX/binutils --with-sysroot --disable-nls --disable-werror
make
make install
```

#### Crossbuild `GCC`
1. Download `GCC` tarball
2. Install necessary dependencies
```
brew install gmp mpfr libmpc
```
2. Build
```
mkdir build-gcc && cd build-gcc
../gcc/configure --target=$TARGET --prefix=$PREFIX/gcc --disable-multilib --disable-nls --enable-languages=c,c++ --without-headers
make all-gcc
make all-target-libgcc
make install-gcc
make install-target-libgcc
```

### Maincourse
1. Clone `GRUB`
```
git clone --depth 1 git://git.savannah.gnu.org/grub.git
```
2. Run `autogen`
```
cd grub && sh autogen.sh
```
> If the script reports missing gettext, install it through Homebrew, or force link it with `brew link gettext --force`
> If the script reports `autoreconf: possibly undefined macro: AC_CHECK_HEADERS`, install `pkg-config` with Homebrew
3. Build
```
mkdir build-grub && cd build-grub
../grub/configure --disable-werror TARGET_CC=$TARGET-gcc TARGET_OBJCOPY=$TARGET-objcopy TARGET_STRIP=$TARGET-strip TARGET_NM=$TARGET-nm TARGET_RANLIB=$TARGET-ranlib --target=$TARGET --prefix=$PREFIX/grub
make
make install
```
4. Install `xorriso`
```
brew install xorriso
```
