FROM archlinux

RUN pacman -Syyu --noconfirm --noprogress

RUN pacman -S kicad xorg-server-xvfb xdotool --noconfirm --noprogress

COPY readme.md /kicad_cli/
COPY Cargo.toml /kicad_cli/
COPY Cargo.lock /kicad_cli/
COPY src /kicad_cli/src

WORKDIR kicad_cli

RUN pacman -S rust gcc --noconfirm --noprogress
RUN cargo build --release
RUN cargo install --path .
ENV PATH=/root/.cargo/bin:$PATH
RUN pacman -R rust gcc --noconfirm

# This is necessary so kicad doesn't pop-up one more window to complain about incorrectly
# set locales
ENV LC_ALL en_US.utf8

# Without some configuration, kicad pops-up a dialog prompting to generate it.
COPY config /root/.config/kicad

VOLUME /workdir
WORKDIR /workdir
