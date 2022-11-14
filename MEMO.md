# RustでUEFIアプリケーションを作成する(Hello Worldの表示)

Rustを用いてUEFIアプリケーションを作成する。
取っ掛かりとして`uefi-rs`クレートを用いたHello Worldを表示するプログラムを作成する。

このメモは上記を通して調べたことなどの備忘録に用いる。

## `rust-toolchain.toml`について

`rust-toolchain.toml`を用いることで、
そのプロジェクト（リポジトリ）限定でtoolchain周りの設定ができる。

### `channel`

使用するコンパイラのリリースの種類を選択する。
以下の種類から選択できる

- `stable`
- `beta`
- `nightly`

また、ここから特定のバージョンを指定することもできる。
`nightly`では機能が崩れたりすることもあるため、
特定のバージョンの指定が重要になることもある？

### `components`

まず、コンポーネントとは`rustc`や`cargo`などを含む
一連のアプリケーションやライブラリのことだと思う。

今回の場合、`rust-src`を指定している。
これは標準ライブラリのソースコードをダウンロードする。

## `.cargo/config.toml`

cargoを使用している際のビルド周りの設定を行うためのファイル。
例えば、毎回入力する必要があるようなビルドオプションをこのファイルに指定することで
ビルド時には自動でそのオプションを指定してくれる。

### `target`

```
[build]
target = "x86_64-unknown-uefi"
```

ビルドターゲットを選択する。
ビルドターゲットは以下のような形式になる

```
<arch><sub>-<vendor>-<sys>-<abi>
```

例えば、`<arch>`の部分には`x86_64`や`arm`などのアーキテクチャが入る。

### `unstable`

## OVMFについて

[OVMFについての参考ページ](https://gihyo.jp/admin/serial/01/ubuntu-recipe/0441)

Qemuなどで使用することができるFLOSS(Free Liberty Open Source Software, 用はオープンソースソフトウェア)な実装。Qemuでは普段はレガシーBIOSである`SeaBIOS`が動くようになっているが、`-drive if=pflash`オプションで指定することでUEFIを動かすことができるようになる。

## UEFI System Partitionについて

System PartitionはUEFIにおけるbootable partitionのこと。(MBRにおける１セクタ目のこと？)
System PartitionはファイルシステムとしてFATを含んでおり、
`/EFI/BOOT`以下のパスにアーキテクチャに合わせたファイル名の実行体を自動的に実行する。

