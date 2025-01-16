@echo off
del .\\library.dll
del .\\loader.exe

cargo build
move .\\target\\debug\\library.dll .\\library.dll
move .\\target\\debug\\loader.exe .\\loader.exe