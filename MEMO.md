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

## Handleについて

Handleとはプログラム上でハードウェアや実行中のバイナリなどの資源を表す。
このHandleはOpaque pointerであるため、利用するにはプロトコルを利用する必要がある。

### Opaque pointerとは

[Opaque Pointerについて参考ページ](https://minus9d.hatenablog.com/entry/2016/01/13/213751)

ざっくりとした理解だと、実体を直接扱わないポインタ。
構造体などでは実装をユーザから隠蔽することができる利点がある。

例えば、以下はOpaque PointerをC言語の関数に利用している例になる（はず）

```
void foo(struct obj* opaque) {
    ...
}
```

## Protocolについて

Protocolとは計算機上の各種資源とやりとりするためのインタフェースである。
例えば、`LoadedImage`や`BlockIO`などのプロトコルが存在する。

ProtocolはBoot Serviceステージの間で利用可能であり、
Runtimeステージでは利用できない。

### ステージとは

UEFIにおけるOSが起動するまでのいくつかの段階のことを指す。
以下の３つのステージが定義されている。

- Platform Initialization
- Boot Services
- Runtime

#### Platform Initialization

`uefi-rs`を利用して記述するプログラムよりも前に動くステージ。
UEFI Platform Initialization Specificationという仕様で記載されている。

#### Boot Services

主に`uefi-rs`を利用して作成するプログラムが動くステージ。
Protocolはこのステージで利用可能である。
また、このステージでOSカーネルのロードを行うなど次ステージへの準備も行われる。

`BootServices`と`RuntimeServices`の両方のが利用可能である。

`uefi-rs`では`SystemTable::exit_boot_services`を呼び出すことで次のステージであるRuntimenに遷移する。

#### Runtime

いわゆるOSが起動して動作しているステージ。

UEFIの機能は制限されており、`BootServices`が使えなくなっている。`RuntimeServices`は使える。

このステージに入ると計算機を一旦リセットしない限り再度Boot Servicesステージに行くことはできない。

