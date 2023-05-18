init:
  lefthook install

test:
  cargo nextest run

coverage:
  cargo llvm-cov nextest --lcov --output-path coverage/lcov.info
  
install:
  cargo install cw-optimizoor cargo-nextest cargo-llvm-cov taplo-cli --locked
  go install github.com/evilmartians/lefthook@latest

get-submodules:
  git submodule init
  git submodule update --remote --merge