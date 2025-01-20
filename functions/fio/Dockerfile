FROM docker.io/ubuntu:24.04

SHELL ["/bin/bash", "-c"]
ARG DEBIAN_FRONTEND=noninteractive

RUN apt update \
    && apt install -y fio python3 python3-pip python3.12-venv

COPY benchmark.sh /
COPY file_gen_1.sh /
COPY file_gen_2.sh /

RUN chmod +x benchmark.sh \
    && chmod +x file_gen_1.sh \
    && chmod +x file_gen_2.sh \
    && source file_gen_1.sh \
    && source file_gen_2.sh \
    && source benchmark.sh

RUN mkdir -p /app \
    && cd /app \
    && python3 -m venv venv \
    && source ./venv/bin/activate \
    && pip3 install Flask

WORKDIR /app
COPY ./app.py /app/app.py
COPY ./app.py /app/app.py
CMD ["/bin/bash", "-c", "source ./venv/bin/activate && python3 ./app.py"]
