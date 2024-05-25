此项目用于下载油管链接的音频并上传到TG机器人
使用Rust编写，需要安装Rust 
## 安装Rust
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
## 构建
cargo build -r
cp target/release/send_tele /usr/bin
chmod +x /usr/bin/send_tele
## 运行
$ send_tele 油管链接 用户ID BOT_TOKEN