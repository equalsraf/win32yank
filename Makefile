all:
	cargo build --release --target x86_64-pc-windows-gnu
redhat_env: # for RedHat Linux 8.x
	sudo dnf install -y mingw64-winpthreads-static
arch_env:  # for Arch Linux
	sudo pacman -S --needed mingw-w64-winpthreads mingw-w64-gcc
