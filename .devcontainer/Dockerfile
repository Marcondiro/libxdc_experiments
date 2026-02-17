FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y \
    python3-matplotlib python3-msgpack python3-numpy git python3 python curl \
    gcc make libipt2 build-essential cmake
RUN curl https://bootstrap.pypa.io/pip/2.7/get-pip.py --output /get-pip.py && \
    python2 /get-pip.py && \
    rm /get-pip.py && \
    pip install msgpack

RUN git clone --depth 1 --single-branch --branch v4 \ 
    https://github.com/capstone-engine/capstone.git && \
    cd capstone/ && \
    chmod u+x ./make.sh && \
    ./make.sh   && \
    ./make.sh install && \
    cd .. && rm -r capstone

# git clone https://github.com/intelxed/xed.git

# RUN pwd && ls -l && cd experiments/libxdc && \
#     # bash prepare_eval.sh && \
#     chmod u+x ./compile.sh && \
#     ./compile.sh || \
#     cd libxdc/ && \
#     mv libxdc.a build/libxdc.a && \
#     mv libxdc.so build/libxdc.so && \
#     cd .. && \
#     ./compile.sh
#     # python3 run_eval.py
