docker run --rm ^
    -e BIN=lambda ^
    -v %CD%:/code ^
    -v %USERPROFILE%/.cargo/registry:/root/.cargo/registry ^
    softprops/lambda-rust
