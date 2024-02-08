# 使用するベースイメージを指定します。Rustの公式イメージを使用します。
FROM rust:latest as builder

# 作業ディレクトリを設定します。
WORKDIR /usr/src/rocket

# 依存関係をキャッシュするために、Cargo.toml と Cargo.lock をコピーします。
# ただし、Cargo.lock がない場合はスキップしてください。
COPY Cargo.toml ./

# ダミーのソースファイルを作成して、依存関係だけをビルドします。
# これにより、ソースコードの変更がない限り、依存関係の再コンパイルを避けられます。
RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release

# 本物のソースコードをコピーします。
COPY ./src ./src
COPY Rocket.toml ./
COPY local.db ./

# ダミーのソースファイルを削除して、本物のソースファイルでアプリケーションを再ビルドします。
ENV DATABASE_URL=sqlite:./local.db
RUN touch src/main.rs && \
    cargo build --release

# 実行ステージを設定します。ビルドステージでコンパイルしたバイナリを軽量なイメージにコピーします。
#FROM debian:buster-slim
FROM debian:12

# SSL証明書をインストールします。これは、外部APIとの安全な通信に必要な場合があります。
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# builder ステージからコンパイル済みバイナリをコピーします。
COPY --from=builder /usr/src/rocket/target/release/rocket .
# ローカルのデータベースファイルをコンテナにコピーします。
COPY local.db .

# Rocketが使用するポートを開放します。
EXPOSE 8080

# 環境変数を設定します。Rocketの設定ファイルや環境変数に応じて調整してください。
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV DATABASE_URL=sqlite:./local.db

# アプリケーションを実行します。
CMD ["./rocket"]
