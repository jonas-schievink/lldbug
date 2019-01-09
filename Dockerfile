FROM base/archlinux

WORKDIR /segfault
RUN yes | pacman -Syq rustup gcc lld
RUN ln -s $(which ld.lld) /usr/local/bin/ld
RUN rustup default nightly-2019-01-08

ENV PATH=/usr/local/bin:/usr/bin
ENV RUST_BACKTRACE=1

# use a single codegen unit and no incr. comp. to narrow down the problem
ENV RUSTFLAGS=-Ccodegen-units=1
ENV CARGO_INCREMENTAL=0

COPY . /segfault

CMD ["cargo", "run"]

