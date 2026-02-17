git clone https://github.com/nyx-fuzz/libxdc.git
cd libxdc
make
mv libxdc.a build/libxdc.a
mv libxdc.so build/libxdc.so
cd -
make
