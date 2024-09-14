test:
    cargo nextest run

covtest:
    cargo llvm-cov nextest
