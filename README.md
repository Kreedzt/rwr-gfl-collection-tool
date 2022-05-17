# RWR 集合工具

## 快速上手

1. 下载[构建后代码](https://github.com/Kreedzt/rwr-gfl-collection-tool/releases)
2. 解压后双击 `rwr-collection-tool-*` 文件夹内 `rwr-collection-tool.exe`, 可直接运行

> 若提示缺少 dll, 安装构建下载页面的 `vc_redist.x64.exe` 即可


## 开发

> 该项目同时依赖 [RWR GFL mod 武器数据提取工具](https://github.com/Kreedzt/rwr-gfl-weapon-parser) 与 [RWR GFL mod 护甲数据提取工具](https://github.com/Kreedzt/rwr-gfl-armor-parser), 需克隆俩项目到与此项目同级目录

> 克隆后的目录结构如下:

``` txt
|- rwr-gfl-collection-tool(本项目)
|- rwr-gfl-weapon-parser
|- rwr-gfl-armor-parser
```

1. 该项目采用 Rust 语言编写，需要 [Rust](https://www.rust-lang.org/) 开发环境
2. 该项目基于 Qt5 开发, 还需要 [Qt](https://www.qt.io/download-open-source) 环境
3. Rust 与 Qt5 安装完毕后, 具体开发环境配置见 [Rust + Qt guide](https://rust-qt.github.io/qt/)
4. 一切就绪后, 使用如下命令进行开发环境编译

``` sh
cargo run
```

## 构建
> 该项目同时依赖 [RWR GFL mod 武器数据提取工具](https://github.com/Kreedzt/rwr-gfl-weapon-parser) 与 [RWR GFL mod 护甲数据提取工具](https://github.com/Kreedzt/rwr-gfl-armor-parser), 需克隆俩项目到与此项目同级目录

> 克隆后的目录结构如下:

``` txt
|- rwr-gfl-collection-tool(本项目)
|- rwr-gfl-weapon-parser
|- rwr-gfl-armor-parser
```

1. 该项目采用 Rust 语言编写，需要 [Rust](https://www.rust-lang.org/) 开发环境
2. 该项目基于 Qt5 开发, 还需要 [Qt](https://www.qt.io/download-open-source) 环境
3. Rust 与 Qt5 安装完毕后, 具体开发环境配置见 [Rust + Qt guide](https://rust-qt.github.io/qt/)
4. 一切就绪后, 使用如下命令进行本项目编译

``` sh
cargo build --release
```

5. 以上代码仅编译了本项目文件, 还需要动态链接 Qt 文件, 具体做法见: [Deployment](https://rust-qt.github.io/qt/deployment/)

例: 在已配置 Qt 环境变量的终端中, 可执行如下命令(假设 deploy 目录已创建)

``` sh
# 以 windows 为例

# 复制编译后的文件到部署目录(部署目录手动创建)
cp .\target\release\rwr-gfl-mod-collection-tool.exe deploy\

# 复制协议文件到部署目录
cp .\LICENSE deploy\

# 使用 Qt 提供的部署工具, 复制必要文件到部署目录
windeployqt.exe .\deploy\rwr-gfl-mod-collection-tool.exe --dir deploy --release --no-translations --no-angle --no-opengl-sw --no-quick-import --no-virtualkeyboard --no-compiler-runtime --no-webkit2
```

以上操作完毕后, 双击 `deploy\rwr-collection-tool.exe` 即可成功运行

> 如果不能保证运行机器包含运行环境, 建议执行 `windeployqt` 时去掉 `--no-compiler-runtime` 参数, 这样会复制一份运行时到部署目录中

## 其他项目

- [RWR GFL mod 武器数据提取工具](https://github.com/Kreedzt/rwr-gfl-weapon-parser)
- [RWR GFL mod 护甲数据提取工具](https://github.com/Kreedzt/rwr-gfl-armor-parser)

## 协议

- [GPLv3](https://opensource.org/licenses/GPL-3.0)
- Qt5: [GPLv3](https://opensource.org/licenses/GPL-3.0)
- 护甲提取工具: [GPLv3](https://opensource.org/licenses/GPL-3.0)
- 武器提取工具: [GPLv3](https://opensource.org/licenses/GPL-3.0)
