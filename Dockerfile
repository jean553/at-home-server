FROM debian:stretch

# required by the base-image to start SSH service and generates default host keys
ENV DEBBASE_SSH enabled

RUN apt-get update -y && \
    apt-get upgrade -y

RUN apt-get install -y \
    python-dev \
    python-pip  \
    libffi-dev \
    libssl-dev \
    pkg-config \
    sudo && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN pip install --upgrade \
    ansible \
    setuptools \
    packaging \
    pyparsing \
    appdirs

COPY at-home-server/Cargo.toml /var/opt/at-home-server/Cargo.toml
COPY at-home-server/src /var/opt/at-home-server/src
COPY at-home-server/provisioning /var/opt/at-home-server/provisioning

RUN ansible-playbook /var/opt/at-home-server/provisioning/site.yml -c local

CMD ["/var/opt/at-home-server/target/release/at-home-server"]

EXPOSE 8000
