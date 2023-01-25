# toyrvemu-rs

## 0. 环境配置

```
❯ cargo new toyrvemu-rs
❯ cargo run
...
Hello, world!
```

WSL2 Ubuntu 20.04 

```
sudo vim /etc/apt/sources.list
```


在源中增加如下内容：

    deb http://apt.llvm.org/focal/ llvm-toolchain-focal main
    deb-src http://apt.llvm.org/focal/ llvm-toolchain-focal main
    # 14
    deb http://apt.llvm.org/focal/ llvm-toolchain-focal-14 main
    deb-src http://apt.llvm.org/focal/ llvm-toolchain-focal-14 main
    # 15
    deb http://apt.llvm.org/focal/ llvm-toolchain-focal-15 main
    deb-src http://apt.llvm.org/focal/ llvm-toolchain-focal-15 main


```
sudo apt update
sudo apt-get install clang-14 lldb-14 lld-14 llvm
```

比较耗时，大约 1h 。


## 1. 支持 Adder 

- 读取指令并解析，支持 add，addi 两条指令。


