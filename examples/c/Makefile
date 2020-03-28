CXX := clang++
CFLAGS := -g -Wall -std=c++11
LDFLAGS := -L../../debug
LIBS := -lpokebell
SRCS := main.cpp

all: build

cargo:
	cargo build --target-dir ../..

build: cargo
	$(CXX) -o example $(CFLAGS) $(LDFLAGS) $(LIBS) $(SRCS)
