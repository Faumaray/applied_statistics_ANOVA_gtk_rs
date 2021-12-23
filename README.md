# Readme for this program

## Later this gonna be guide for how to Build, Install and Use program

This program was created within the framework of teaching the discipline _Applied Statistics_ by students of the _MOIS-91_ group

## Guide for developers on Windows

1. Install [MSYS2](https://www.msys2.org)
2. Install **_GCC_** and all necessary system headers from instructions on **_Mingw x64_** console `pacman -S --needed base-devel mingw-w64-x86_64-toolchain`
3. Install on **_Mingw x64_** console **_Gtk4_** and **_Poppler_** `pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-poppler`
   > ### Alternative №1
   >
   > Install [Rust](https://www.rust-lang.org) on mingw64 `pacman -S mingw-w64-x86_64-rust`
   >
   > ### Alternative №2
   >
   > Add to PATH variable "_path to MSYS2_\mingw64\bin"
   > Install [Rust](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-gnu/rustup-init.exe)
4. To build run in root directory of project, if used Atlernative №1 run command using **_Mingw x64_** console
   1. Development version `cargo build`
   2. Release version `cargo build --release`
5. You can find builded binary in **Out** folder

## Guide for developers on Linux

1. If not installed install _GCC_ using your package manager
2. Install **_Gtk4_** and **_Poppler_** using your package manager
3. Install [Rust](https://www.rust-lang.org)
4. To build run in root directory of project
   1. Development version `cargo build`
   2. Release version `cargo build --release`
5. You can find builded binary in **Out** folder

---

## Гайд для разработчиков на Windows

1. Установить [MSYS2](https://www.msys2.org)
2. Следуя инструкциям установить **_GCC_** и все необходимые системыне зависимости используя **_Mingw x64_** консоль `pacman -S --needed base-devel mingw-w64-x86_64-toolchain`
3. Установить **_Gtk4_** и **_Poppler_** используя **_Mingw x64_** консоль `pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-poppler`
   > ### Альтернатива №1
   >
   > Установить [Rust](https://www.rust-lang.org) используя **_Mingw x64_** консоль `pacman -S mingw-w64-x86_64-rust`
   >
   > ### Альтернатива №2
   >
   > Добавить в системную переменную _PATH_: "_путь до папки MSYS2_\mingw64\bin"
   > Установить [Rust](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-gnu/rustup-init.exe)
4. Для компиляции программы запустить в корне проекта, если использовалась Альтернатива №1, использовать "**_Mingw x64_** консоль"
   1. Версия для отладки `cargo build`
   2. Релизная версия `cargo build --release`
5. Скомпилированный файл можно найти в папке: **Out**

## Гайд для разработчиков на Linux

1. Если не установлен _GCC_, установите используя свой пакетный менеджер
2. Установите библиотеки **_Gtk4_** и **_Poppler_**, используя свой пакетный менеджер
3. Установите [Rust](https://www.rust-lang.org)
4. Для компиляции программы запустить в корне проекта
   1. Версия для отладки `cargo build`
   2. Релизная версия `cargo build --release`
5. Скомпилированный файл можно найти в папке: **Out**
