@echo off

echo Building release version...
cargo build --release

echo Creating installer...

rem path to the Inno Setup Compiler
set ISCC="C:\Program Files (x86)\Inno Setup 6\ISCC.exe"

if exist %ISCC% (
    %ISCC% scripts/installer.iss
    echo Installer created successfully!
) else (
    echo Error: Inno Setup Compiler not found at %ISCC%
    echo Please install Inno Setup or update the ISCC path
    exit /b 1
)
