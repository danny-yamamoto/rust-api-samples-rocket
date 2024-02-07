# ベースイメージとしてRustの最新安定版を使用
FROM rust:latest as builder

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# 依存関係のキャッシュを最適化するためにCargo.tomlとCargo.lockを先にコピー
COPY rocket/Cargo.toml rocket/Cargo.lock ./

# ダミーのソースファイルを作成して依存関係をビルド
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/myapp*

# 実際のソースコードをコピー
COPY rocket/src ./src
COPY rocket/Rocket.toml ./Rocket.toml

# アプリケーションをビルド
RUN cargo build --release

# 実行環境の準備
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# ビルドステップからコンパイルされたバイナリをコピー
COPY --from=builder /usr/src/myapp/target/release/myapp .

# ローカルデータベースファイルをコピー（必要であれば）
COPY rocket/local.db .

# 8080ポートを開放
EXPOSE 8080

# アプリケーションを実行
CMD ["./myapp"]
