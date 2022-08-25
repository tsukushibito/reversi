FROM rust
RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get install -y --no-install-recommends \
    zsh python3.9 python3-pip

ARG USER_NAME=reversi
ARG USER_ID=1000
ARG GROUP_NAME=reversi
ARG GROUP_ID=1000

RUN groupadd -g ${GROUP_ID} ${GROUP_NAME}
RUN useradd -u ${USER_ID} -g ${GROUP_ID} -d /home/${USER_ID} --create-home --shell /usr/bin/zsh ${USER_NAME}

USER ${USER_ID}
ENV HOME /home/${USER_NAME}
WORKDIR ${HOME}

RUN rustup component add rustfmt
RUN rustup component add clippy

RUN pip3 install torch torchvision torchaudio --extra-index-url https://download.pytorch.org/whl/cpu