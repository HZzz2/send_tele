# 📥 YouTube 音频下载并上传到 Telegram 机器人

此项目用于下载 YouTube 链接的音频并上传到 Telegram 机器人。项目使用 Rust 编写，因此需要先安装 Rust。

## 🚀 安装 Rust

在终端中运行以下命令以安装 Rust：

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 🔧 构建

构建项目并将可执行文件复制到 `/usr/bin` 目录：

```sh
$ cargo build -r
$ cp target/release/send_tele /usr/bin
$ chmod +x /usr/bin/send_tele
```

## ▶️ 运行

使用以下命令运行程序：

```sh
$ send_tele <油管链接> <用户ID> <BOT_TOKEN>
```

其中：
- `<油管链接>` 是要下载音频的 YouTube 视频链接。
- `<用户ID>` 是要发送音频的 Telegram 用户 ID。
- `<BOT_TOKEN>` 是您的 Telegram 机器人的令牌。

通过这个项目，您可以轻松地将 YouTube 音频下载并发送到 Telegram。🎉

如果有任何问题或建议，欢迎提交 issue 或 pull request！✨

