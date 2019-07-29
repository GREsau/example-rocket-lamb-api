docker run --rm \
    -e BIN=lambda \
    -v ${PWD}:/code \
    -v ${HOME}/.cargo/registry:/root/.cargo/registry \
    softprops/lambda-rust
