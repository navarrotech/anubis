@echo off
if "%1" == "init" (
    cargo run init
) else (
    echo Unknown command: %1
)
