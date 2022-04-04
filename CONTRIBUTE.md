# 贡献文档

## 安装 cargo-watch

```bash
cargo install cargo-watch
```

## 运行 wasm 项目

```bash
cargo watch -C "src" -s "wasm-pack build"
```

## 运行 demo 项目

```bash
cd demo
npm i
npm start
```

## npm link

```bash
cd pkg
npm link
cd ../demo
npm link imagequant-wasm
```

## 预览

打开 http://localhost:8080/