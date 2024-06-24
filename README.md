# rust-cookbook

Rust Cookbook

## 算法(algorithms)
```sh
cargo new algorithms
cd algorithms && mkdir src/bin

```
### 生成随机值

```sh
cargo add rand
cargo add rand_distr

mkdir src/bin
touch src/bin/ch01_01.rs

cargo run --bin ch01_01
```

### Vector 排序

```sh
touch src/bin/ch01_02.rs

```

## 命令行(cli)
```sh
cargo new cli
cd cli && mkdir src/bin
cargo add clap
cd ..
```

### 参数解析 clap

```sh
cargo add clap
# version 2
touch src/bin/ch02_01.rs

```

### 参数解析 bpaf

```sh
cargo add bpaf --features bpaf_derive
touch src/bin/ch02_02.rs

```

### ANSI 终端

用 ansi_term 控制 ANSI 终端上的颜色和格式，如蓝色粗体文本或黄色下划线文本。

```sh
cargo add ansi_term
touch src/bin/ch02_03.rs
cargo run --bin ch02_03
```

## 压缩(compression)

```sh
cargo new compression
cd compression && mkdir src/bin
cargo add flate2 tar
touch src/bin/ch03_01.rs
cargo build --bin ch03_01
/target/debug
./ch03_01
```

## 并发/并行(concurrency)
```sh
cargo new concurrency
cd concurrency && mkdir src/bin
cargo add crossbeam crossbeam_channel lazy_static error_chain threadpool num_cpus walkdir ring num image
cd ..
### 显式线程

- 生成短期线程

本实例使用 crossbeam crate 为并发和并行编程提供了数据结构和函数。Scope::spawn 生成一个新的作用域线程，该线程确保传入 crossbeam::scope 函数的闭包在返回之前终止，这意味着您可以从调用的函数中引用数据。

```sh
touch src/bin/ch04_01.rs
cargo run --bin ch04_01
```

- 创建并发的数据管道

下面的实例使用 crossbeam 和 crossbeam-channel 两个 crate 创建了一个并行的管道，与 ZeroMQ 指南 中所描述的类似：管道有一个数据源和一个数据接收器，数据在从源到接收器的过程中由两个工作线程并行处理。

```sh
touch src/bin/ch04_01_02.rs
cargo run --bin ch04_01_02
```

- 在两个线程间传递数据
  这个实例示范了在单生产者、单消费者（SPSC）环境中使用 crossbeam-channel。

```sh
touch src/bin/ch04_01_03.rs
cargo run --bin ch04_01_03
```

- 保持全局可变状态
  使用 lazy_static 声明全局状态。lazy_static 创建了一个全局可用的 static ref，它需要 Mutex 来允许变化（请参阅 RwLock）。在 Mutex 的包裹下，保证了状态不能被多个线程同时访问，从而防止出现争用情况。必须获取 MutexGuard，方可读取或更改存储在 Mutex 中的值。

```sh
touch src/bin/ch04_01_04.rs
cargo run --bin ch04_01_04
```

- 对所有 iso 文件的 SHA256 值并发求和
  计算了当前目录中每个扩展名为 iso 的文件的 SHA256 哈希值。线程池生成的线程数与使用 num_cpus::get 获取的系统内核数相等。Walkdir::new 遍历当前目录，并调用 execute 来执行读取和计算 SHA256 哈希值的操作。

```sh
touch src/bin/ch04_01_05.rs
cargo run --bin ch04_01_05
```

- 将绘制分形的线程分派到线程池
  此实例通过从朱莉娅集绘制分形来生成图像，该集合具有用于分布式计算的线程池。
  使用 ImageBuffer::new 为指定宽度和高度的输出图像分配内存，Rgb::from_channels 信道则计算输出图像的 RGB 像素值。使用 ThreadPool 创建线程池，线程池中的线程数量和使用 num_cpus::get 获取的系统内核数相等。ThreadPool::execute 将每个像素作为单独的作业接收。

mpsc::channel 信道接收作业，Receiver::recv 接收器则检索作业。ImageBuffer::put_pixel 处理数据，设置像素颜色。最后，ImageBuffer::save 将图像存储为 output.png。

```sh
cargo add num image
touch src/bin/ch04_01_06.rs
cargo run --bin ch04_01_06
```

### 数据并行

- 并行改变数组中元素
  rayon 数据并行库,为任何并行可迭代的数据类型提供 par_iter_mut 方法。这是一个类迭代器的链，可以对链内的数据并行计算。

```sh
cargo add rayon
touch src/bin/ch04_02_01.rs
cargo run --bin ch04_02_01
```

- 并行测试集合中任意或所有的元素是否匹配给定断言
  这个实例示范如何使用 rayon::any 和 rayon::all 方法，这两个方法是分别与 std::any 和 std::all 相对应的并行方法。rayon::any 并行检查迭代器的任意元素是否与断言匹配，并在找到一个匹配的元素时就返回。rayon::all 并行检查迭代器的所有元素是否与断言匹配，并在找到不匹配的元素时立即返回。

```sh
touch src/bin/ch04_02_02.rs
cargo run --bin ch04_02_02
```

- 使用给定断言并行搜索项
  下面的实例使用 rayon::find_any 和 par_iter 并行搜索 vector 集合，以查找满足指定闭包中的断言的元素。

如果有多个元素满足 rayon::find_any 闭包参数中定义的断言，rayon 将返回搜索发现的第一个元素，但不一定是 vector 集合的第一个元素。

请注意，实例中闭包的参数是对引用的引用（&&x）。

```sh
touch src/bin/ch04_02_03.rs
cargo run --bin ch04_02_03
```

- 对 vector 并行排序
  首先，分配空字符串 vector；然后，通过 par_iter_mut().for_each 并行对 vector 填充随机值。尽管存在多种选择，可以对可枚举数据类型进行排序，但 par_sort_unstable 通常比稳定排序（相同的值排序后相对顺序不变）算法快。

```sh
cargo add rand
touch src/bin/ch04_02_04.rs
cargo run --bin ch04_02_04
```

- Map-reduce 并行计算
  此实例使用 rayon::filter、rayon::map，以及 rayon::reduce 计算 Person 对象中年龄超过 30 岁的那些人的平均年龄。

rayon::filter 过滤集合中满足给定断言的元素。rayon::map 对每个元素执行一次计算，创建一个新的迭代；然后，基于前一次的 reduce 计算结果和当前元素一起，rayon::reduce 执行新的计算。也可以查看 rayon::sum，它与本实例中的 reduce 计算具有相同的结果。

```sh
touch src/bin/ch04_02_05.rs
cargo run --bin ch04_02_05
```

- 并行生成 jpg 缩略图
  为当前目录中的所有 .jpg 图像文件生成缩略图，然后将生成的缩略图保存在一个名为 thumbnails 的新文件夹中。

```sh
cargo add glob
touch src/bin/ch04_02_06.rs
cargo run --bin ch04_02_06
```

## 密码学(cryptography)
```sh
cargo new cryptography
cd cryptography && mkdir src/bin
cargo add data-encoding
cd ..

### 散列(哈希)

- 计算文件的 SHA-256 摘要
  先创建文件，写入一些数据。然后使用 digest::Context 计算文件内容的 SHA-256 摘要 digest::Digest。

```sh
touch src/bin/ch05_01_01.rs
cargo run --bin ch05_01_01
# SHA-256 digest is 81700022B5CAB8EFC79F276B69D17251B03FFCDAB61C026B75F783B55E3953CB
```

- 使用 HMAC 摘要对消息进行签名和验证
  使用 ring::hmac 创建字符串的签名 hmac::Signature，然后验证签名是否正确。

```sh
touch src/bin/ch05_01_02.rs
cargo run --bin ch05_01_02
```

### 加密

- 使用 PBKDF2 对密码进行加密（salt）和散列（hash）运算
  对于通过 PBKDF2 密钥派生函数 pbkdf2::derive 生成的加密（加盐算法）密码，使用 ring::pbkdf2 进行散列（哈希）运算，使用 pbkdf2::verify 验证散列（哈希）运算是否正确。salt 值是使用 SecureRandom::fill 生成的，salt 字节数组被其安全生成的随机数填充。

```sh
touch src/bin/ch05_02.rs
cargo run --bin ch05_02
# Salt: E19CD56CA8C1666483186F88175D5C832F50E7F5E7528593546D79C5DBD3CBCDC6AE20353613C4AF677C11CD89374BDFD34E5C920205FFB6D37BC18167B2AE7A
# PBKDF2 hash: 04F6222550F81FDC6880EADF778B4216687D3580F05C6E2478ACAEE63FA03CEDC9F53F91DDFE59D21F3145D44B60382D2C6CD4C824C78115F8DBB754DE069965
```

## 数据结构(data_structures)
```sh
cargo new data_structures
cd data_structures
cargo add bitflags
```
### 位域

- 定义并操作位域风格的类型
  在 bitflags! 宏的帮助下创建类型安全的位域类型 MyFlags，并为其实现基本的清理操作（clear 方法）以及 Display trait。随后，展示了基本的按位操作和格式化。

```sh

touch src/bin/ch06_01.rs
cargo run --bin ch06_01
```

## 数据库(database)
```sh
cargo new database
cd database
cargo add rusqlite postgres
mkdir src/bin
cd ..
```
### SQLite

- 创建 SQLite 数据库

```sh
touch src/bin/ch07_01_01.rs
cargo run --bin ch07_01_01
```

- 数据插入和查询

```sh
touch src/bin/ch07_01_02.rs
cargo run --bin ch07_01_02
```

- 事务处理

```sh
touch src/bin/ch07_01_03.rs
cargo run --bin ch07_01_03
```

### Postgres

- Postgres 数据库中创建表

```sh
cargo add postgres
touch src/bin/ch07_02_01.rs
cargo run --bin ch07_02_01
```

- 数据插入和查询

```sh
cargo add postgres
touch src/bin/ch07_02_02.rs
cargo run --bin ch07_02_02
```

- 数据聚合

```sh
cargo add postgres
touch src/bin/ch07_02_03.rs
cargo run --bin ch07_02_03
```

## 日期及时间(datetime)
```sh
cargo new datetime
cd datetime
cargo add chrono && mkdir src/bin && cd ..
cargo run --bin ch08_01_01
```
### 期间和计算

- 测量运行时间
  测量从 time::Instant::now 开始运行的时间 time::Instant::elapsed。

调用 time::Instant::elapsed 将返回 time::Duration，将在实例末尾打印该时间。
此方法不会更改或者重置 time::Instant 对象。

ch08_01_01.rs

- 执行日期检查和时间计算
  使用 DateTime::checked_add_signed 计算并显示两周之后的日期和时间，
  使用 DateTime::checked_sub_signed 计算并显示前一天的日期。
  如果无法计算出日期和时间，这些方法将返回 None。

ch08_01_02.rs

- 时间的时区转换
  使用 offset::Local::now 获取本地时间并显示，然后使用 DateTime::from_utc 结构体方法将其转换为 UTC 标准格式。
  最后，使用 offset::FixedOffset 结构体，可以将 UTC 时间转换为 UTC+8 和 UTC-2。

ch08_01_03.rs

### 解析与显示

- 检查日期和时间
  通过 Timelike 获取当前 UTC DateTime 及其时/分/秒，通过 Datelike 获取其年/月/日/工作日。

ch08_02_01.rs

- 日期和 UNIX 时间戳的互相转换
  使用 NaiveDateTime::timestamp 将由 NaiveDate::from_ymd 生成的日期和由 NaiveTime::from_hms 生成的时间转换为 UNIX 时间戳。
  然后，它使用 NaiveDateTime::from_timestamp 计算自 UTC 时间 1970 年 01 月 01 日 00:00:00 开始的 10 亿秒后的日期。

ch08_02_02.rs

- 日期和时间的格式化显示
  使用 Utc::now 获取并显示当前 UTC 时间。
  使用 DateTime::to_rfc2822 将当前时间格式化为熟悉的 RFC 2822 格式，
  使用 DateTime::to_rfc3339 将当前时间格式化为熟悉的 RFC 3339 格式，
  也可以使用 DateTime::format 自定义时间格式。

ch08_02_03.rs

- 将字符串解析为 DateTime 结构体
  熟悉的时间格式 RFC 2822、RFC 3339，以及自定义时间格式，通常用字符串表达。
  要将这些字符串解析为 DateTime 结构体，可以分别用 DateTime::parse_from_rfc2822、DateTime::parse_from_rfc3339，以及 DateTime::parse_from_str。

可以在 chrono::format::strftime 中找到适用于 DateTime::parse_from_str 的转义序列。
注意：DateTime::parse_from_str 要求这些 DateTime 结构体必须是可创建的，以便它唯一地标识日期和时间。
要解析不带时区的日期和时间，请使用 NaiveDate、NaiveTime，以及 NaiveDateTime。

ch08_02_04.rs

## 开发工具(development_tools) TODO
```sh
cargo new development_tools
```

### 调试工具

#### 日志信息

- 记录调试信息到控制台
  log crate 提供了日志工具，
  env_logger crate 通过环境变量配置日志记录。
  log::debug! 宏的工作方式类似于其它 std::fmt 格式化的字符串。
  默认情况下，日志级别为 error

```sh
cd development_tools
cargo add log env_logger
touch src/bin/ch09_01_01.rs
# 设置 RUST_LOG 环境变量以打印消息
RUST_LOG=debug cargo run --bin ch09_01_01
```

- 记录错误信息到控制台
  通过 log 便捷宏 log::error!，将错误记录到 stderr。

```sh
touch src/bin/ch09_01_02.rs
```

- 记录信息时，用标准输出 stdout 替换标准错误 stderr
  使用 Builder::target 创建自定义的日志记录器配置，将日志输出的目标设置为 Target::Stdout。

```sh
touch src/bin/ch09_01_03.rs
```

- 使用自定义日志记录器记录信息
  现一个打印到 stdout 的自定义记录器 ConsoleLogger。为了使用日志宏，ConsoleLogger 实现了 log::Log trait，通过 log::set_logger 安置。

```sh
touch src/bin/ch09_01_04.rs
```

- 记录到 Unix 系统日志
  本实例实现将信息记录到 UNIX syslog。使用 syslog::init 初始化记录器后端。
  syslog::Facility 记录提交日志项分类的程序，log::LevelFilter 表示欲记录日志的等级，Option<&str> 定义应用程序名称（可选）。

```sh
cd development_tools
cargo add syslog
touch src/bin/ch09_01_05.rs
```

#### 日志配置

- 启用每个模块的日志级别

```sh
touch src/bin/ch09_02_01.rs
RUST_LOG="warn,test::foo=info,test::foo::bar=debug"
```

- 用自定义环境变量设置日志记录
  Builder::parse 以 RUST_LOG 语法的形式解析 MY_APP_LOG 环境变量的内容。
  然后，Builder::init 初始化记录器。所有这些步骤通常由 env_logger::init 在内部完成。

```sh
touch src/bin/ch09_02_02.rs
```

- 在日志信息中包含时间戳
  调用 Builder::format 设置一个闭包，该闭包用时间戳、R
  ecord::level 和正文（Record::args）对每个信息文本进行格式化。

```sh
touch src/bin/ch09_02_03.rs
```

- 将信息记录到自定义位置
  log4rs 将日志输出配置到自定义位置。log4rs 可以使用外部 YAML 文件或生成器配置。

使用文件附加器 log4rs::append::file::FileAppender 创建日志配置，文件附加器定义日志记录的目标位置。
日志配置使用 log4rs::encode::pattern 中的自定义模式进行编码，将配置项分配给 log4rs::config::Config，并设置默认的日志等级 log::LevelFilter。

```sh
cargo add log4rs
touch src/bin/ch09_02_04.rs
```

### 版本控制

- 解析并递增版本字符串
  使用 Version::parse 从字符串字面量构造语义化版本 semver::Version，然后逐个递增补丁（修订）版本号、副（次要）版本号和主版本号。

```sh
cd development_tools
cargo add semver
touch src/bin/ch09_03_01.rs
```

- 解析复杂的版本字符串
  使用 Version::parse 从复杂的版本字符串构造语义化版本 semver::Version。该字符串包含语义化版本控制规范中定义的预发布和构建元数据。

```sh
touch src/bin/ch09_03_02.rs
```

- 检查给定版本是否为预发布版本
  给定两个版本，使用 is_prerelease 断言一个是预发布，另一个不是。

```sh
touch src/bin/ch09_03_03.rs
```

- 查询适配给定范围的最新版本
  给定一个版本字符串 &str 的列表，查找最新的语义化版本 semver::Version。semver::VersionReq 用 VersionReq::matches 过滤列表，也可以展示语义化版本 semver 的预发布参数设置。

```sh
touch src/bin/ch09_03_04.rs
```

- 检查外部命令的版本兼容性

本实例使用 Command 模块运行命令 git --version，然后使用 Version::parse 将版本号解析为语义化版本 semver::Version。VersionReq::matches 将 semver::VersionReq 与解析的语义化版本进行比较。最终，命令输出类似于“git version x.y.z”。

```sh
touch src/bin/ch09_03_05.rs
```

### 构建工具

按照惯例，构建时代码存放在 build.rs 文件，通常称为“构建脚本”。

- 编译并静态链接到绑定的 C 语言库
  ，cc crate 提供了一个简单的 API，用于将绑定的 C/C++/asm 代码编译成静态库（.a），静态库可以通过 rustc 静态链接。

```sh
cargo new hello
cd hello
cargo add cc --build
cargo add error-chain
# /Users/andy/2024/github/rust-cookbook/hello
```

Cargo.toml

```toml
[package]
...
build = "build.rs"

[build-dependencies]
cc = "1"

[dependencies]
error-chain = "0.11"

```

build.rs

```rust
fn main() {
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");   // 输出 `libhello.a`
}

```

src/hello.c

```cpp
#include <stdio.h>

void hello() { printf("Hello from C!\n"); }

void greet(const char *name) { printf("哈哈 Hello, %s!\n", name); }

```

- 编译并静态链接到绑定的 C++ 语言库

链接绑定的 C++ 语言库非常类似于链接绑定的 C 语言库。编译并静态链接绑定的 C++ 库时，与链接绑定的 C 语言库相比有两个核心区别：一是通过构造器方法 cpp(true) 指定 C++ 编译器；二是通过在 C++ 源文件顶部添加 extern "C" 代码段，以防止 C++ 编译器的名称篡改。

```sh
cargo new foo
cd foo
cargo add cc --build
# /Users/andy/2024/github/rust-cookbook/foo
```

build.rs

```rust
fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .compile("foo");
}

```

- 编译 C 语言库时自定义设置
  使用 cc::Build::define 自定义构建绑定的 C 语言代码非常简单。
  该方法接受 Option 值，因此可以创建这样的定义：#define APP_NAME "foo"、#define WELCOME（将 None 作为不确定值传递）。
  实例构建了一个绑定的 C 语言文件，其在 build.rs 中设置了动态定义，并在运行时打印 “Welcome to foo - version 1.0.2”。
  Cargo 设定了一些环境变量，这些变量可能对某些自定义设置有用

```sh
cargo new foo2
cd foo2
cargo add cc --build
# /Users/andy/2024/github/rust-cookbook/foo2
```

build.rs

```rust
fn main() {
    cc::Build::new()
        .define("APP_NAME", "\"foo\"")
        .define("VERSION", format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str())
        .define("WELCOME", None)
        .file("src/foo.c")
        .compile("foo");
}

```

## 编码(encoding)
```sh
cargo new encoding
cd encoding && cargo add percent-encoding data-encoding base64 csv serde serde-json byteorder toml url error_chain && cd ..
mkdir encoding/src/bin
cargo run --bin encoding
```
### 字符集
```sh
touch encoding/src/bin/ch10_01_01.rs
touch encoding/src/bin/ch10_01_02.rs
touch encoding/src/bin/ch10_01_03.rs
touch encoding/src/bin/ch10_01_04.rs
cargo run --bin ch10_01_01
```
- 百分比编码（URL 编码）字符串

使用 percent-encoding crate 中的 utf8_percent_encode 函数对输入字符串进行百分比编码（URL 编码）。
解码使用 percent_decode 函数。

encoding/src/bin/ch10_01_01.rs

- 将字符串编码为 application/x-www-form-urlencoded
如下实例使用 form_urlencoded::byte_serialize 将字符串编码为 application/x-www-form-urlencoded 表单语法，随后使用 form_urlencoded::parse 对其进行解码。
这两个函数都返回迭代器，然后这些迭代器聚集为 String。

encoding/src/bin/ch10_01_02.rs

- 编码和解码十六进制

data_encoding crate 提供了 HEXUPPER::encode 方法，该方法接受 &[u8] 参数并返回十六进制数据的字符串 String。
类似地，data_encoding crate 提供了 HEXUPPER::decode 方法，该方法接受 &[u8] 参数。如果输入数据被成功解码，则返回 Vec<u8>。

下面的实例将 &[u8] 数据转换为等效的十六进制数据，然后将此值与预期值进行比较。

encoding/src/bin/ch10_01_03.rs

- 编码和解码 base64
使用 encode 将字节切片编码为 base64 字符串，对 base64 字符串解码使用 decode。
encoding/src/bin/ch10_01_04.rs

### CSV处理
```sh
touch encoding/src/bin/ch10_02_01.rs
touch encoding/src/bin/ch10_02_02.rs
touch encoding/src/bin/ch10_02_03.rs
touch encoding/src/bin/ch10_02_04.rs
touch encoding/src/bin/ch10_02_05.rs
touch encoding/src/bin/ch10_02_06.rs
touch encoding/src/bin/ch10_02_07.rs
touch encoding/src/bin/ch10_02_08.rs
cargo run --bin ch10_02_01
```

- 读取 CSV 记录
将标准的 CSV 记录读入 csv::StringRecord——一种弱类型的数据表示方式，它需要 CSV 中的行数据是有效的 UTF-8 字符编码。
另外，csv::ByteRecord 对 UTF-8 不做任何预设。
encoding/src/bin/ch10_02_01.rs

- 读取 CSV 记录
Serde 将数据反序列化为强类型结构体。具体查阅 csv::Reader::deserialize 方法。
encoding/src/bin/ch10_02_02.rs
```sh
cd encoding
cargo add serde --features derive
```

- 读取有不同分隔符的 CSV 记录
使用制表（tab）分隔符 delimiter 读取 CSV 记录。
encoding/src/bin/ch10_02_03.rs

- 筛选匹配断言的 CSV 记录
仅仅 返回 data 中字段（field）与 query 匹配的的行。
encoding/src/bin/ch10_02_04.rs

- 用 Serde 处理无效的 CSV 数据
CSV 文件通常包含无效数据。对于这些情形，csv crate 提供了一个自定义的反序列化程序 csv::invalid_option，它自动将无效数据转换为 None 值。
encoding/src/bin/ch10_02_05.rs

- 将记录序列化为 CSV
本实例展示了如何序列化 Rust 元组。csv::writer 支持从 Rust 类型到 CSV 记录的自动序列化。write_record 只写入包含字符串数据的简单记录。
具有更复杂值（如数字、浮点和选项）的数据使用 serialize 进行序列化。因为 csv::writer 使用内部缓冲区，所以在完成时总是显式刷新 flush。
encoding/src/bin/ch10_02_06.rs

- 用 Serde 将记录序列化为 CSV
encoding/src/bin/ch10_02_07.rs

- 转换 CSV 文件的列
将包含颜色名称和十六进制颜色值的 CSV 文件转换为具有颜色名称和 rgb 颜色值的 CSV 文件。
使用 csv crate 读写 csv 文件，使用 serde crate 对行输入字节进行反序列化，对行输出字节进行序列化。

详细请参阅 csv::Reader::deserialize、serde::Deserialize，以及 std::str::FromStr。
encoding/src/bin/ch10_02_08.rs

### 结构化数据
```sh
touch encoding/src/bin/ch10_03_01.rs
touch encoding/src/bin/ch10_03_02.rs
touch encoding/src/bin/ch10_03_03.rs
touch encoding/src/bin/ch10_03_04.rs
cargo run --bin ch10_03_01
```

- 对非结构化 JSON 序列化和反序列化
serde_json crate 提供了 from_str 函数来解析 JSON 切片 &str。
非结构化 JSON 可以被解析为一个通用的 serde_json::Value 类型，该类型能够表示任何有效的 JSON 数据。
下面的实例展示如何解析 JSON 切片 &str，期望值被 json! 宏声明。
encoding/src/bin/ch10_03_01.rs

- 反序列化 TOML 配置文件
将一些 TOML 配置项解析为一个通用的值 toml::Value，该值能够表示任何有效的 TOML 数据。
encoding/src/bin/ch10_03_02.rs

- 反序列化 TOML 配置文件
使用 Serde crate 将 TOML 解析为自定义的结构体。
encoding/src/bin/ch10_03_03.rs

- 以小端模式（低位模式）字节顺序读写整数
字节序 byteorder 可以反转结构化数据的有效字节。当通过网络接收信息时，这可能是必要的，例如接收到的字节来自另一个系统。
encoding/src/bin/ch10_03_04.rs

## 错误处理(errors)
### 处理错误变量
```sh
cargo new errors
cd errors && cargo add error_chain reqwest && cd ..
mkdir errors/src/bin
touch errors/src/bin/ch11_01.rs
touch errors/src/bin/ch11_02.rs
touch errors/src/bin/ch11_03.rs
cargo run --bin errors
```

- 在 main 方法中对错误适当处理
errors/src/bin/ch11_01.rs

error-chain crate 包含大量的模板代码，用于 Rust 中的错误处理。

foreign_links 代码块内的 Io(std::io::Error) 函数允许由 std::io::Error 所报错误信息到 error_chain! 所定义错误类型的自动转换，error_chain! 所定义错误类型将实现 Error trait。

- 避免在错误转变过程中遗漏错误
errors/src/bin/ch11_02.rs
error-chain crate 使得匹配函数返回的不同错误类型成为可能，并且相对简洁。ErrorKind 是枚举类型，可以确定错误类型。

实例使用 reqwest::blocking 来查询一个随机整数生成器的 web 服务，并将服务器响应的字符串转换为整数。
Rust 标准库 reqwest 和 web 服务都可能会产生错误，所以使用 foreign_links 定义易于辨认理解的 Rust 错误。
另外，用于 web 服务错误信息的 ErrorKind 变量，使用 error_chain! 宏的 errors 代码块定义。

- 获取复杂错误场景的回溯
errors/src/bin/ch11_03.rs
依赖于 chain_err，通过附加新的错误来扩展错误信息。从而可以展开错误堆栈，这样提供了更好的上下文来理解错误的产生原因。
尝试将值 256 反序列化为 u8。首先 Serde 产生错误，然后是 csv，最后是用户代码。

可以通过附加命令参数 RUST_BACKTRACE=1 运行实例，以显示与此错误相关的详细回溯。

## 文件系统(file)
```sh
cargo new file
cd file && cargo add same_file walkdir glob memmap && cd ..
mkdir file/src/bin
cargo run --bin file
```
### 文件读写
```sh
touch file/src/bin/ch12_01_01.rs
touch file/src/bin/ch12_01_02.rs
touch file/src/bin/ch12_01_03.rs
cargo run --bin ch12_01_01
```
- 读取文件的字符串行

向文件写入三行信息，然后使用 BufRead::lines 创建的迭代器 Lines 读取文件，一次读回一行。
File 模块实现了提供 BufReader 结构体的 Read trait。
File::create 打开文件 File 进行写入，File::open 则进行读取。
file/src/bin/ch12_01_01.rs

- 避免读取写入同一文件

对文件使用 same_file::Handle 结构体，可以测试文件句柄是否等同。
在本实例中，将对要读取和写入的文件句柄进行相等性测试。
file/src/bin/ch12_01_02.rs

```sh
cargo run --bin ch12_01_02 >>new.txt
```

- 使用内存映射随机访问文件

使用 memmap 创建文件的内存映射，并模拟文件的一些非序列读取。
使用内存映射意味着您仅需索引一个切片，而不是使用 seek 方法来导航整个文件。
Mmap::map 函数假定内存映射后的文件没有被另一个进程同时更改，否则会出现竞态条件。
file/src/bin/ch12_01_03.rs

### 目录遍历
```sh
touch file/src/bin/ch12_02_01.rs
touch file/src/bin/ch12_02_02.rs
touch file/src/bin/ch12_02_03.rs
touch file/src/bin/ch12_02_04.rs
touch file/src/bin/ch12_02_05.rs
touch file/src/bin/ch12_02_06.rs
touch file/src/bin/ch12_02_07.rs
touch file/src/bin/ch12_02_08.rs
```

- 过去 24 小时内修改过的文件名

通过调用 env::current_dir 获取当前工作目录，然后通过 fs::read_dir 读取目录中的每个条目，通过 DirEntry::path 提取条目路径，以及通过 fs::Metadata 获取条目元数据。
Metadata::modified 返回条目自上次更改以来的运行时间 SystemTime::elapsed。
Duration::as_secs 将时间转换为秒，并与 24 小时（24 * 60 * 60 秒）进行比较。
Metadata::is_file 用于筛选出目录。
file/src/bin/ch12_02_01.rs

```sh
cd file && cargo add error_chain && cd ..
cargo run --bin ch12_02_01
```

- 查找给定路径的循环

使用 same_file::is_same_file 检测给定路径的循环。例如，可以通过软连接（符号链接）在 Unix 系统上创建循环：
```sh
mkdir -p /tmp/foo/bar/baz
ln -s /tmp/foo/  /tmp/foo/bar/baz/qux

```
下面的实例将断言存在一个循环。
file/src/bin/ch12_02_02.rs

- 递归查找重名文件
在当前目录中递归查找重复的文件名，只打印一次。

file/src/bin/ch12_02_03.rs

- 使用给定断言递归查找所有文件
在当前目录中查找最近一天内修改的 JSON 文件。
使用 follow_links 确保软链接（符号链接）像普通目录和文件一样被按照当前查找规则执行。
file/src/bin/ch12_02_04.rs

- 跳过隐藏文件遍历目录
递归下行到子目录的过程中，使用 filter_entry 对目录中的条目传递 is_not_hidden 断言，从而跳过隐藏的文件和目录。
Iterator::filter 可应用到要检索的任何目录 WalkDir::DirEntry，即使父目录是隐藏目录。
根目录 "." 的检索结果，通过在断言 is_not_hidden 中使用 WalkDir::depth 参数生成。
file/src/bin/ch12_02_05.rs

- 在给定深度的目录，递归计算文件大小
通过 WalkDir::min_depth 和 WalkDir::max_depth 方法，可以灵活设置目录的递归深度。
下面的实例计算了 3 层子文件夹深度的所有文件的大小总和，计算中忽略根文件夹中的文件。
file/src/bin/ch12_02_06.rs

- 递归查找所有 png 文件
递归地查找当前目录中的所有 PNG 文件。在本实例中，** 模式用于匹配当前目录及其所有子目录。
在路径任意部分使用 ** 模式，例如，/media/**/*.png 匹配 media 及其子目录中的所有 PNG 文件。
file/src/bin/ch12_02_07.rs

- 忽略文件名大小写，使用给定模式查找所有文件
在 /media/ 目录中查找与正则表达模式 img_[0-9]*.png 匹配的所有图像文件。
一个自定义 MatchOptions 结构体被传递给 glob_with 函数，使全局命令模式下不区分大小写，同时保持其他选项的默认值 Default。
file/src/bin/ch12_02_08.rs

## 硬件支持(hardware)
### 处理器
```sh
cargo new hardware
cd hardware && cargo add num_cpus && cd ..
cargo run --bin hardware
```

- 检查逻辑 cpu 内核的数量
使用 [num_cpus::get] 显示当前机器中的逻辑 CPU 内核的数量。
main.rs

## 内存管理(mem)
### 全局静态/全局堆栈
```sh
cargo new mem
cd mem && cargo add lazy_static
cargo run --bin mem
```
- 声明延迟计算常量
声明延迟计算的常量 HashMap。HashMap 将被计算一次，随后存储在全局静态（全局堆栈）引用。
main.rs

## 网络(net)
### 服务器
```sh
cargo new net
cargo run --bin net
```
- 监听未使用的 TCP/IP 端口

在本实例中，程序将监听显示在控制台上的端口，直到一个请求被发出。当将端口设置为 0 时，SocketAddrV4 会分配一个随机端口。
main.rs

## 操作系统(os)

```sh
cargo new os
mkdir os/src/bin
cd os
cargo add regex
cargo run os
```

- 运行外部命令并处理 stdout
将 git log --oneline 作为外部命令 Command 运行，并使用 Regex 检查其 Output，以获取最后 5 次提交的哈希值和消息。

```sh
touch os/src/bin/ch16_01_01.rs
cargo run --bin ch16_01_01
```

- 运行传递 stdin 的外部命令，并检查错误代码
使用外部命令 Command 打开 python 解释器，并传递一条 python 语句供其执行，然后解析语句的输出结构体 Output。

```sh
touch os/src/bin/ch16_01_02.rs
cargo run --bin ch16_01_02
```

- 运行管道传输的外部命令
显示当前工作目录中前 10 大的文件和子目录，它等同于运行： du -ah . | sort -hr | head -n 10。
每个命令 Command 代表一个进程，子进程的输出是通过父进程和子进程之间的管道 Stdio::piped 捕获的。
```sh
touch os/src/bin/ch16_01_03.rs
cargo run --bin ch16_01_03
```

- 将子进程的 stdout 和 stderr 重定向到同一个文件
生成子进程并将 stdout 和 stderr 重定向到同一个文件。它遵循与运行管道传输的外部命令相同的思想，但是 process::Stdio 会将输出写入指定的文件。对 stdout 和 stderr 而言，File::try_clone 引用相同的文件句柄。它将确保两个句柄使用相同的光标位置进行写入。

下面的实例等同于运行 Unix shell 命令 ls . oops >out.txt 2>&1。
```sh
touch os/src/bin/ch16_01_04.rs
cargo run --bin ch16_01_04
```

- 持续处理子进程的输出
在运行外部命令并处理-stdout 实例中，直到外部命令 Command 完成，stdout 的处理才开始。下面的实例调用 Stdio::piped 创建管道，并在 BufReader 被更新后立即读取 stdout，持续不断地处理。

下面的实例等同于 Unix shell 命令 journalctl | grep usb。

```sh
touch os/src/bin/ch16_01_05.rs
cargo run --bin ch16_01_05
```

- 读取环境变量
通过 std::env::var 读取环境变量。

```sh
touch os/src/bin/ch16_01_06.rs
cargo run --bin ch16_01_06
```

## 科学计算(science)
```sh
cargo new science
mkdir science/src/bin
cd science
cargo add ndarray
```

### 数学
#### 线性代数
```sh
touch src/bin/ch17_01_01.rs
touch src/bin/ch17_01_02.rs
touch src/bin/ch17_01_03.rs
touch src/bin/ch17_01_04.rs
touch src/bin/ch17_01_05.rs
touch src/bin/ch17_01_06.rs
touch src/bin/ch17_01_07.rs
cargo run --bin ch17_01_01
```

- 矩阵相加
使用 ndarray::arr2 创建两个二维（2-D）矩阵，并按元素方式求和。
src/bin/ch17_01_01.rs

- 矩阵相乘
使用 ndarray::arr2 创建两个矩阵，并使用 ndarray::ArrayBase::dot 对它们执行矩阵乘法。
src/bin/ch17_01_02.rs

- 标量、vector、矩阵相乘
使用 ndarray::arr1 创建一维（1-D）数组（vector），使用 ndarray::arr2 创建二维（2-D）数组（矩阵）。

首先，一个标量乘以一个 vector 得到另一个 vector。然后，使用 ndarray::Array2::dot 将矩阵乘以新的 vector（矩阵相乘使用 dot 函数，而 * 运算符执行元素方式的乘法）。

在 ndarray crate 中，根据上下文，一维数组可以解释为行 vector 或列 vector。如果 vector 表示的方向很重要，则必须使用只有一行或一列的二维（2-D）数组。在本实例中，vector 是右侧的一维（1-D）数组，因此 dot 函数将其处理为列 vector。

src/bin/ch17_01_03.rs

- Vector 比较
ndarray crate 支持多种创建数组的方法——此实例使用 from 从 std::Vec 创建数组 ndarray::Array。然后，对数组以元素方式求和。

下面的实例按元素方式比较两个浮点型 vector。浮点数的存储通常不精确，因此很难进行精确的比较。但是，approx crate 中的 assert_abs_diff_eq! 宏允许方便地比较浮点型元素。要将 approx 和 ndarray 两个 crate一起使用，必须在 Cargo.toml 文件中的 ndarray 依赖项添加 approx 特性。例如：ndarray = { version = "0.13", features = ["approx"] }。
src/bin/ch17_01_04.rs
```sh
cargo add approx  
cargo add ndarray --features approx
```

- Vector 范数
这个实例展示了 Array1 类型、ArrayView1 类型、fold 方法，以及 dot 方法在计算给定 vector 的 l1 和 l2 范数时的用法。 + l2_norm 函数是两者中较简单的，它计算一个 vector 与自身的点积（dot product，数量积）的平方根。 + l1_norm 函数通过 fold 运算来计算元素的绝对值（也可以通过 x.mapv(f64::abs).scalar_sum() 执行，但是会为 mapv 的结果分配一个新的数组）。

src/bin/ch17_01_05.rs

- 矩阵求逆
用 nalgebra::Matrix3 创建一个 3x3 的矩阵，如果可能的话，将其求逆。
src/bin/ch17_01_06.rs

- （反）序列化矩阵
本实例实现将矩阵序列化为 JSON，以及从 JSON 反序列化出矩阵。序列化由 serde_json::to_string 处理，serde_json::from_str 则执行反序列化。

src/bin/ch17_01_07.rs

#### 三角学
```sh
touch src/bin/ch17_02_01.rs
touch src/bin/ch17_02_02.rs
touch src/bin/ch17_02_03.rs
cargo run --bin ch17_02_01
```

- 计算三角形的边长
计算直角三角形斜边的长度，其中斜边的角度为 2 弧度，对边长度为 80。
src/bin/ch17_02_01.rs

- 验证正切（tan）等于正弦（sin）除以余弦（cos）
验证 tan(x) 是否等于 sin(x)/cos(x)，其中 x=6。
src/bin/ch17_02_02.rs

- 地球上两点之间的距离
使用半正矢公式计算地球上两点之间的距离（以公里为单位）。两个点用一对经纬度表示，然后，to_radians 将它们转换为弧度。sin、cos、powi 以及 sqrt 计算中心角。最终，可以计算出距离。
src/bin/ch17_02_03.rs

#### 复数
```sh
cargo add num
touch src/bin/ch17_03_01.rs
touch src/bin/ch17_03_02.rs
touch src/bin/ch17_03_03.rs
cargo run --bin ch17_03_01
```

- 创建复数
创建类型 num::complex::Complex 的复数，复数的实部和虚部必须是同一类型。
src/bin/ch17_03_01.rs

- 复数相加
对复数执行数学运算与对内置类型执行数学运算是一样的：计算的数字必须是相同的类型（如浮点数或整数）
src/bin/ch17_03_02.rs

- 复数的数学函数
在与其他数学函数交互时，复数有一系列有趣的特性，尤其是和自然常数 e（欧拉数）类似的正弦相关函数。
src/bin/ch17_03_03.rs

#### 统计学
```sh
touch src/bin/ch17_04_01.rs
touch src/bin/ch17_04_02.rs
touch src/bin/ch17_04_03.rs
touch src/bin/ch17_04_04.rs
cargo run --bin ch17_04_01
```

计算 Rust 数组中包含的数据集的集中趋势度量。对于一个空的数据集，可能没有平均数、中位数或众数去计算，因此每个函数都返回 [Option] ，由调用者处理。

- 集中趋势度量

第一个实例是通过对数据引用生成一个迭代器，然后计算平均数（所有测量值的总和除以测量值的计数），并使用 [sum] 和 [len] 函数分别确定值的总和及值的计数。
src/bin/ch17_04_01.rs

- 集中趋势度量
使用快速选择算法（quick select algorithm）计算中位数，该算法只对已知可能包含中位数的数据集的分区进行排序，从而避免了完整[排序][sort]。
该算法使用 [cmp] 和 [Ordering] 简便地地决定要检查的下一个分区，并使用 [split_at] 为每个步骤的下一个分区选择一个任意的枢轴量。
src/bin/ch17_04_02.rs

- 集中趋势度量
使用可变的 [HashMap] 来计算众数，[fold] 和 [entry] API 用来从集合中收集每个不同整数的计数。[HashMap] 中最常见的值可以用 [max_by_key] 取得。
src/bin/ch17_04_03.rs

- 计算标准偏差
本实例计算一组测量值的标准偏差和 z 分数（z-score）。

标准偏差定义为方差的平方根（用 f32 浮点型的 [sqrt] 计算），其中方差是每个测量值与平均数之间的平方差的和除以测量次数。

z 分数（z-score）是指单个测量值偏离数据集平均数的标准差数。
src/bin/ch17_04_04.rs

#### 其他数学计算
```sh
touch src/bin/ch17_05_01.rs
cargo run --bin ch17_05_01
```

- 大数
BigInt 使得超过 128 位的大整数计算成为可能。
src/bin/ch17_05_01.rs

## 文本处理(text)
```sh
cargo new text
cd text && cargo add regex lazy_static unicode-segmentation 
mkdir src/bin && cd ..
```
### 正则表达式

- 验证并提取电子邮件登录信息

验证电子邮件地址的格式是否正确，并提取 @ 符号之前的所有内容。

ch18_01_01.rs

- 从文本提取标签元素唯一的列表
  本实例展示从文本中提取、排序和去除标签列表的重复元素。
  这里给出的标签正则表达式只捕获以字母开头的拉丁语标签，完整的 twitter 标签正则表达式要复杂得多。

ch18_01_02.rs

- 从文本提取电话号码
  使用 Regex::captures_iter 处理一个文本字符串，以捕获多个电话号码。这里的例子中是美国电话号码格式。

ch18_01_03.rs

- 通过匹配多个正则表达式来筛选日志文件
  读取名为 application.log 的文件，并且只输出包含下列内容的行：“version X.X.X”、端口为 443 的 IP 地址（如 “192.168.0.1:443”）、特定警告。
  正则表达集构造器 regex::RegexSetBuilder 构建了正则表达式集 regex::RegexSet。由于反斜杠在正则表达式中非常常见，因此使用原始字符串字面量可以使它们更具可读性。

ch18_01_04.rs

- 文本模式替换
  将所有出现的国际标准 ISO 8601 日期模式 YYYY-MM-DD 替换为具有斜杠的等效美式英语日期模式。例如： 2013-01-15 替换为 01/15/2013。
  正则表达式的使用，需要了解匹配规则：全文匹配（match regex）、搜索匹配（search regex）、替换匹配（replace regex）。

ch18_01_05.rs

### 字符串解析

- 收集 Unicode 字符
  使用 unicode-segmentation crate 中的 UnicodeSegmentation::graphemes 函数，从 UTF-8 字符串中收集个别的 Unicode 字符。

ch18_02_01.rs

- 自定义结构体并实现 FromStr trait

创建一个自定义结构体 RGB 并实现 FromStr trait，以将提供的颜色十六进制代码转换为其 RGB 颜色代码。

ch18_02_02.rs
The RGB color code is: R: 250 G: 114 B: 104

## Web 编程(web)
```sh
cargo new web
cd web && cargo add reqwest select url regex && mkdir src/bin && cd ..
```
### 抓取网页

- 从 HTML 网页中提取所有链接

使用 reqwest::get 执行 HTTP GET 请求，然后使用 Document::from_read 将响应信息解析为 HTML 文档。以“a”（锚元素）作为结构体 Name 的参数，将结构体 Name 作为条件，使用 find 方法检索所有链接。
在结构体 Selection 上调用 filter_map 方法，从具有 “href” attr（属性）的链接检索所有 URL。

```sh
cd web
cargo add tokio --features full
touch src/bin/ch19_01_01.rs
cargo run --bin ch19_01_01
```

- 检查网页死链
  调用 get_base_url 方法检索 base URL，如果 HTML 文档有 base 标签，从 base 标记获取 href attr，初始 URL 的默认值是 Position::BeforePath。

遍历 HTML 文档中的链接，并创建一个 tokio::spawn 任务，该任务将使用 url::ParseOptions 结构体和 Url::parse 方法解析单个链接。任务执行中，使用 reqwest 向链接发起请求，并验证状态码结构体 StatusCode。实例中使用 await 异步等待任务完成，然后结束程序。

```sh
cd web
cargo add error_chain
touch src/bin/ch19_01_02.rs
cargo run --bin ch19_01_02
```

- 从 MediaWiki 标记页面提取所有唯一性链接
  使用 reqwest::get 获取 MediaWiki 页面的源代码，然后使用 Regex::captures_iter 查找内部和外部链接的所有条目。
  使用智能指针 Cow 可以提供对借用数据的不可变引用，避免分配过多的字符串。

```sh
touch web/src/bin/ch19_01_03.rs
# 要翻墙
cargo run --bin ch19_01_03
```

### URL

- 解析 URL 字符串为 Url 类型
  url crate 中的 parse 方法验证并解析 &str 切片为 Url 结构体。如果输入字符串的格式不正确，解析方法 parse 会返回 Result<Url, ParseError>。

一旦 URL 被解析，它就可以使用 Url 结构体类型中的所有方法。

```sh
touch src/bin/ch19_02_01.rs
cargo run --bin ch19_02_01
```

- 通过移除路径段创建基本 URL
  基本 URL（base URL）包括协议和域名。但基本 URL（base URL）不包括目录、文件或查询字符串，这些项都可以从给定的 URL 中剥离出来。创建基本 URL（base URL）时，通过 PathSegmentsMut::clear 方法移除目录和文件路径，通过方法 Url::set_query 移除查询字符串。

```sh
touch src/bin/ch19_02_02.rs
cargo run --bin ch19_02_02
```

- 从基本 URL 创建新 URLs
  join 方法从基路径和相对路径创建新的 URL。

```sh
touch src/bin/ch19_02_03.rs
cargo run --bin ch19_02_03
```

- 提取 URL 源（scheme / host / port）
  Url 结构体定义了多种方法，以便于提取有关它所表示的 URL 的信息。

```sh
touch src/bin/ch19_02_04.rs
cargo run --bin ch19_02_04
```

- 从 URL 移除片段标识符和查询对
  解析 Url 结构体，并使用 url::Position 枚举对其进行切片，以去除不需要的 URL 片段。

```sh
touch src/bin/ch19_02_05.rs
cargo run --bin ch19_02_05
```

### 媒体类型

- 从字符串获取 MIME 类型
  使用 mime crate 从字符串解析出 MIME 类型。FromStrError 结构体在 unwrap_or 子句中生成默认的 MIME 类型。

```sh
cargo add mime
touch src/bin/ch19_03_01.rs
cargo run --bin ch19_03_01
```

- 从文件名获取 MIME 类型
  使用 mime crate 从给定的文件名返回正确的 MIME 类型。
  程序将检查文件扩展名并与已知的 MIME 类型列表匹配，返回值为 mime:Mime。

```sh
touch src/bin/ch19_03_02.rs
cargo run --bin ch19_03_02
```

- 解析 HTTP 响应的 MIME 类型
  当从 reqwest 接收到 HTTP 响应时，MIME 类型或媒体类型可以在实体头部的 Content-Type 标头中找到。
  reqwest::header::HeaderMap::get 方法将标头检索为结构体 reqwest::header::HeaderValue，结构体可以转换为字符串。
  然后 mime crate 可以解析它，生成 mime::Mime 值。

```sh
touch src/bin/ch19_03_03.rs
cargo run --bin ch19_03_03
```

### 客户端

### 请求处理

- 发出 HTTP GET 请求
  解析提供的 URL，并使用 reqwest::blocking::get 发起同步 HTTP GET 请求。
  打印获取的响应消息状态和标头 reqwest::blocking::Response。
  使用 read_to_string 将 HTTP 响应消息主体正文读入到指派的字符串 String。

```sh
cd web
cargo add reqwest --features blocking
touch src/bin/ch19_04_01.rs
cargo run --bin ch19_04_01
```

- 为 REST 请求设置自定义消息标头和 URL 参数
  为 HTTP GET 请求设置标准的和自定义的 HTTP 消息标头以及 URL 参数。使用 hyper::header! 宏创建 XPoweredBy 类型的自定义消息标头。

```sh
cd web
cargo add reqwest --features json,multipart
cargo add serde
cargo add hyper --features full
touch src/bin/ch19_04_02.rs
cargo run --bin ch19_04_02
```

#### Web API 调用

- 查询 GitHub API
  使用 reqwest::get 查询 点赞的用户 API v3，以获取某个 GitHub 项目的所有点赞用户的列表。使用 Response::json 将响应信息 reqwest::Response 反序列化为实现了 serde::Deserialize trait 的 User 对象。

```sh
touch src/bin/ch19_05_01.rs
cargo run --bin ch19_05_01
```

- 检查 API 资源是否存在
  使用消息标头 HEAD 请求（(Client::head）查询 GitHub 用户端接口，然后检查响应代码以确定是否成功。这是一种无需接收 HTTP 响应消息主体，即可快速查询 rest 资源的方法。
  使用 ClientBuilder::timeout 方法配置的 reqwest::Client 结构体将确保请求不会超时。

```sh
touch src/bin/ch19_05_02.rs
cargo run --bin ch19_05_02
```

- 使用 GitHub API 创建和删除 Gist
  使用 Client::post 创建一个 POST 请求提交到 GitHub gists API v3 接口的 gist，并使用 Client::delete 使用 DELETE 请求删除它。
  实例中使用 HTTP 基本认证 为了授权访问 GitHub API。实际应用中或许将使用一个更为复杂的 OAuth 授权流程。

```sh
cd web
cargo add serde
touch src/bin/ch19_05_03.rs
cargo run --bin ch19_05_03
```

- 使用 RESTful API 分页
  可以将分页的 web API 方便地包裹在 Rust 迭代器中，当到达每一页的末尾时，迭代器会从远程服务器加载下一页结果。

```sh
touch src/bin/ch19_05_04.rs
cargo run --bin ch19_05_04
```

- 处理速率受限 API
  使用 GitHub API - 速率限制展示如何处理远程服务器错误。
  本实例使用 hyper::header! 宏来解析响应头并检查 reqwest::StatusCode::Forbidden。如果响应超过速率限制，则将等待并重试。

```sh
touch src/bin/ch19_05_05.rs
cargo run --bin ch19_05_05
```

#### 下载

- 下载文件到临时目录
  使用 tempfile::Builder 创建一个临时目录，并使用 reqwest::get 通过 HTTP 协议异步下载文件。

使用 Response::url 方法内部的 tempdir() 方法获取文件名字，使用 File 结构体创建目标文件，并使用 io::copy 将下载的数据复制到文件中。程序退出时，会自动删除临时目录。

```sh
cd web
cargo add tempdir tempfile
touch src/bin/ch19_06_01.rs
cargo run --bin ch19_06_01
```

- 使用 HTTP range 请求头进行部分下载
  使用 reqwest::blocking::Client::head 获取响应的消息主体的大小（即消息主体内容长度）。

然后，使用 reqwest::blocking::Client::get 下载 10240 字节的内容，同时打印进度消息。本实例使用同步的 reqwest 模块，消息范围标头指定响应的消息块大小和位置。

RFC7233 中定义了消息范围标头。

```sh
touch src/bin/ch19_06_02.rs
cargo run --bin ch19_06_02
```

- POST 文件到 paste-rs
  使用 reqwest::Client 建立与 https://paste.rs 的连接，遵循 reqwest::RequestBuilder 结构体模式。
  调用 Client::post 方法，以 URL 为参数连接目标，RequestBuilder::body 通过读取文件设置要发送的内容，RequestBuilder::send 方法在文件上传过程中将一直阻塞，直到返回响应消息。最后，read_to_string 返回响应消息并显示在控制台中。

```sh
touch src/bin/ch19_06_03.rs
cargo run --bin ch19_06_03
```
