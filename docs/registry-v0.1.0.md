# crates.io 0.1.0 发布验收

日期：2026-10-22。

- 包名：`metrics233-client-sdk-rust`，版本：`0.1.0`，最低 Rust：`1.88`。
- 不可变源代码：`3cb5fedf6e26514a134a593a3a6de4b6325ffb4d`，对应 GitHub `v0.1.0`。
- SHA-256：`209401a867d0a0787e76eb36f6f481c96e24a43505743b45592ebf410b99eb2a`。
- 原传输依赖固定为 `metrics233 = 0.1.0`，本包只有一个实现入口。

## 发布和安装

[crates.io](https://crates.io/crates/metrics233-client-sdk-rust/0.1.0) 已返回可下载、未撤回的正式版本。注册表校验和、GitHub 发行包和独立 Cargo 缓存中实际下载的 `.crate` 字节一致。

```sh
cargo add metrics233-client-sdk-rust@0.1.0
cargo add tokio --features rt-multi-thread,macros
```

上传使用 `cargo publish --locked --no-verify --registry crates-io`，复用已通过 [Rust 1.88 标签 CI](https://github.com/neko233-com/metrics233-client-sdk-rust/actions/runs/39768672606) 和打包验证的相同文件。上传前再次比对了 SHA-256。没有修改既有标签、版本或发行包。

## 独立消费者

新项目通过正式注册表安装，不使用 Git/path 依赖或 `[patch]`。独立 Cargo home 的下载校验通过，锁文件的来源为 `registry+https://github.com/rust-lang/crates.io-index`。

运行验收已通过：Darwin arm64 全新项目（Rust/Cargo 1.99.1）→ loopback SSH 通道 → 隔离的真实 `metrics233-server` embedded 后端。使用合成凭据和测试指标，没有写入生产指标。

- `cargo add metrics233-client-sdk-rust@0.1.0 --registry crates-io` 成功；锁文件来源和下载包 SHA-256 与正式版本一致。
- 新项目原生编译、累计 Counter、Gauge 和显式 shutdown/ACK 全部成功。服务端重新打开存储后回读 `sdk_registry_calls_total = 5`、`sdk_registry_gauge = 233`。
- 错误上报 Token 返回 401，对应拒绝指标无样本；缺少 Token 返回 401；上报 Token 访问管理读取返回 403。
- SDK 及其 `metrics233` 传输依赖由官方注册表独立下载。为缩短慢网络等待，另外三个公共依赖压缩包从已校验的独立缓存补齐；没有复用旧项目编译产物，也没有 Git/path 替换。

Windows 本次复验受页面文件不足（OS error 1455）影响，不能计作成功编译；标签 CI 和此前真实服务验收单独保留。
