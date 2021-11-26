# Readme for this program 
## Later this gonna be guide for how to Build, Install and Use program

This program was created within the framework of teaching the discipline *Applied Statistics* by students of the *MOIS-91* group

## Guide for developers on Windows
1. install [Build Tools](https://aka.ms/vs/17/release/vs_BuildTools.exe) with package for C++ Development
2. install [vcpkg](https://vcpkg.io/en/getting-started.html) 
3. install in vcpkg gtk `vcpkg install gtk:x64-windows`
4. install [pkg-config](https://sourceforge.net/projects/pkgconfiglite/files/latest/download) and add to path his *bin* folder
5. Add to variables *PKG_CONFIG_PATH* = "*vcpkg dir*\installed\x64-windows\lib\pkgconfig"
6. run command *vcpkg integrate install*
7. install [Rust](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe) with toolchain *nightly-x86_64-pc-windows-msvc*

# Attention for collaborators for your development use own branch and make pull requests. DO NOT USE MAIN BRANCH!!!