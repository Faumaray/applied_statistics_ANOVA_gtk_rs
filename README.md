# Readme for this program 
## Later this gonna be guide for how to Build, Install and Use program

This program was created within the framework of teaching the discipline *Applied Statistics* by students of the *MOIS-91* group

## Guide for developers on Windows
1. install Build Tools with package for C++ Development
2. install vcpkg 
3. install in vcpkg gtk
4. install pkg-config
5. Add to variables *PKG_CONFIG_PATH* = "*vcpkg dir*\installed\x64-windows\lib\pkgconfig"
6. run command *vcpkg integrate install*
7. install RUst with toolchain *nightly-x86_64-pc-windows-msvc*

# Attention for collaborators for your development use own branch and make pull requests. DO NOT USE MAIN BRANCH!!!