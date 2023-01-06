# toyrvemu-rs

## 0. 

## 1. 环境配置

```
❯  cargo new toyrvemu-rs
❯ cargo run
...
Hello, world!
```

WSL2 Ubuntu 20.04 

下载 [clang-12](https://github.com/llvm/llvm-project/releases/tag/llvmorg-12.0.0) 解压到 `/usr/local` 路径下。

```
cd /usr/local
sudo tar xvf clang+llvm-12.0.0-x86_64-linux-gnu-ubuntu-20.04.tar.xz
sudo mv clang+llvm-12.0.0-x86_64-linux-gnu-ubuntu-20.04 llvm
export PATH="$PATH:/usr/local/llvm/bin"
```

注意不要下错版本，否则无法通过测试！

