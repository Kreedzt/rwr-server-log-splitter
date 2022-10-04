# RWR Server log 分割工具

![license](https://badgen.net/github/license/Kreedzt/rwr-server-log-splitter)
![latest release](https://badgen.net/github/release/Kreedzt/rwr-server-log-splitter)
![commits count](https://badgen.net/github/commits/Kreedzt/rwr-server-log-splitter)
![last commit](https://badgen.net/github/last-commit/Kreedzt/rwr-server-log-splitter)
![rwr_version](https://badgen.net/badge/RWR/1.94/orange)

## 使用

去 [Release](https://github.com/Kreedzt/rwr-server-log-splitter/releases) 页面下载二进制包.

在启动 `rwr_server` 之前启动该程序即可.

该程序会自动监听 `rwr_server.log` 的变化, 并按日期时间将 log 信息分片储存在当前目录的 `watch-log` 下

分片储存的文件格式为: `rwr_server.log-{{yy}}-{{mm}}-{{dd}}-{{hh}}-{{mm}}-{{ss}}`

## 开发

该项目采用 Rust 语言编写，需要 [Rust](https://www.rust-lang.org/) 开发环境

启动开发环境的命令:

``` sh
cargo run
```

在当前路径下编辑 `rwr_server.log`, 会在 `watch-log` 文件夹下生成分片 log 文件 

## 构建

该项目采用 Rust 语言编写，需要 [Rust](https://www.rust-lang.org/) 开发环境

编译需执行以下命令：
```bash
cargo build --release
```

编译后在根目录的 `target/release` 内生成二进制文件（exe），该文件可用终端直接运行。

## 协议

- [MIT](https://opensource.org/licenses/MIT)
