# Base image
ARG BASE_IMAGE=debian
ARG VARIANT=bookworm-slim
FROM debian:${VARIANT}
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# Arguments
ARG CONTAINER_USER=esp
ARG CONTAINER_GROUP=esp
ARG ESP_BOARD=all
#ARG GITHUB_TOKEN
ARG XTENSA_VERSION=latest
#ARG XTENSA_VERSION=1.77.0.0

# Install dependencies
RUN apt-get -o Acquire::Check-Valid-Until=false  -o Acquire::Check-Date=false update \
    && apt-get install -y git curl gcc clang ninja-build unzip libudev-dev tar xz-utils \
    libusb-1.0-0 libssl-dev pkg-config python3 python3-pip python3-venv libpython3-all-dev python3-virtualenv\
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

# Install lsusb so we can access host com ports
RUN apt-get -o Acquire::Check-Valid-Until=false  -o Acquire::Check-Date=false update \
    && apt-get install -y sudo usbip hwdata usbutils \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

# Set users
RUN adduser --disabled-password --gecos "" ${CONTAINER_USER} && \
    usermod -a -G plugdev ${CONTAINER_USER} && \
    usermod -a -G dialout ${CONTAINER_USER} && \
    usermod -a -G tty ${CONTAINER_USER}
RUN echo "${CONTAINER_USER} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
RUN mkdir -p /home/${CONTAINER_USER}/.vscode-server /home/${CONTAINER_USER}/.vscode-server-insiders /home/${CONTAINER_USER}/.vscode-server/extensions /home/${CONTAINER_USER}/.vscode-server-insiders/extensions /home/${CONTAINER_USER}/history /home/${CONTAINER_USER}/.ssh/ \
    && chown -R ${CONTAINER_USER} /home/${CONTAINER_USER}/.vscode-server /home/${CONTAINER_USER}/.vscode-server-insiders /home/${CONTAINER_USER}/.vscode-server/extensions /home/${CONTAINER_USER}/.vscode-server-insiders/extensions /home/${CONTAINER_USER}/history /home/${CONTAINER_USER}/.ssh/ \
    && SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/home/${CONTAINER_USER}/history/.bash_history" \
    && touch /home/${CONTAINER_USER}/history/.bash_history \
    && chown -R ${CONTAINER_USER} /home/${CONTAINER_USER}/history \
    && echo $SNIPPET >> "/home/${CONTAINER_USER}/.bashrc" \
    && chmod -R 600 /home/${CONTAINER_USER}/.ssh/ \
    && chmod 700 /home/${CONTAINER_USER}/.ssh
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain none -y --profile minimal

# Update envs
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin

# Install extra crates
RUN ARCH=$($HOME/.cargo/bin/rustup show | grep "Default host" | sed -e 's/.* //') && \
    curl -L "https://github.com/esp-rs/espup/releases/latest/download/espup-${ARCH}" -o "${HOME}/.cargo/bin/espup" && \
    chmod u+x "${HOME}/.cargo/bin/espup" && \
    curl -L "https://github.com/esp-rs/embuild/releases/latest/download/ldproxy-${ARCH}.zip" -o "${HOME}/.cargo/bin/ldproxy.zip" && \
    unzip "${HOME}/.cargo/bin/ldproxy.zip" -d "${HOME}/.cargo/bin/" && \
    rm "${HOME}/.cargo/bin/ldproxy.zip" && \
    chmod u+x "${HOME}/.cargo/bin/ldproxy" && \
    curl -L "https://github.com/esp-rs/espflash/releases/latest/download/cargo-espflash-${ARCH}.zip" -o "${HOME}/.cargo/bin/cargo-espflash.zip" && \
    unzip "${HOME}/.cargo/bin/cargo-espflash.zip" -d "${HOME}/.cargo/bin/" && \
    rm "${HOME}/.cargo/bin/cargo-espflash.zip" && \
    chmod u+x "${HOME}/.cargo/bin/cargo-espflash" && \
    curl -L "https://github.com/esp-rs/espflash/releases/latest/download/espflash-${ARCH}.zip" -o "${HOME}/.cargo/bin/espflash.zip" && \
    unzip "${HOME}/.cargo/bin/espflash.zip" -d "${HOME}/.cargo/bin/" && \
    rm "${HOME}/.cargo/bin/espflash.zip" && \
    chmod u+x "${HOME}/.cargo/bin/espflash" && \
    curl -L "https://github.com/esp-rs/esp-web-flash-server/releases/latest/download/web-flash-${ARCH}.zip" -o "${HOME}/.cargo/bin/web-flash.zip" && \
    unzip "${HOME}/.cargo/bin/web-flash.zip" -d "${HOME}/.cargo/bin/" && \
    rm "${HOME}/.cargo/bin/web-flash.zip" && \
    chmod u+x "${HOME}/.cargo/bin/web-flash"

RUN mkdir -p /home/${CONTAINER_USER}/.local/share/bash-completion/completions \
    && espup completions bash > /home/${CONTAINER_USER}/.local/share/bash-completion/completions/espup

# Install Rust toolchain for our ESP_BOARD
# RUN if [ -n "${GITHUB_TOKEN}" ]; then export GITHUB_TOKEN=${GITHUB_TOKEN}; fi && \
RUN version="" && \
    if [ "${XTENSA_VERSION}" != "latest" ];then version="--toolchain-version ${XTENSA_VERSION}"; fi && \
    ${HOME}/.cargo/bin/espup install\
    --targets "${ESP_BOARD}" \
    --log-level debug \
    --export-file /home/${CONTAINER_USER}/export-esp.sh \
    $version

# Activate ESP environment
RUN echo "source /home/${CONTAINER_USER}/export-esp.sh" >> ~/.bashrc

# Set default toolchain
RUN if [ "${ESP_BOARD}" = "all" ] || echo "$ESP_BOARD" | grep -q "esp32c" || echo "$ESP_BOARD" | grep -q "esp32h"; then \
    rustup default nightly; \
    rustup component add rustfmt ; \
    else \
    rustup default esp; \
    fi

CMD [ "/bin/bash" ]
