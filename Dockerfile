FROM ubuntu:19.04
ENV TZ Europe/Moscow
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN mkdir -p /app
RUN apt-get update && apt-get --yes upgrade && apt-get install --yes bash vim curl libopencv-dev libssl-dev
ENV CARGO_HOME=/opt/cargo
ENV RUSTUP_HOME=/opt/rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN /opt/cargo/bin/rustup install nightly
RUN /opt/cargo/bin/rustup default nightly
COPY ./test-task-trlogic /app/test-task-trlogic
COPY ./libs /app/libs
COPY ./Cargo.toml /app
COPY ./Cargo.lock /app
WORKDIR /app
RUN /opt/cargo/bin/cargo test
CMD /opt/cargo/bin/cargo run -- /app/test-task-trlogic/config/config.yaml