# md_parser

簡単なタスク管理を行える markdown parser です。

# 使い方

```
$ cargo build
$ md_parser <markdown_file>
```

# 出力例

例えば、以下のような markdown ファイルでメモを書いていたとします。

```
# sample

# (2024/04/01 09:30) *TODO* タスク 1
## (2024/04/10 09:30) *DONE* サブタスク 1.1
## (2024/04/20 09:30) *WIP* サブタスク 1.2

待ち。
*WAIT* (2024/04/25 09:30) 

## (2024/05/01 09:30) サブタスク 1.3

(2024/05/03 09:30) 仕掛中 *WIP*

# タスク 2

ラベルなし。
```

これを md_parser で処理すると、以下のような出力になります。

```
$ markdown.exe sample.md
WAIT items:
  sample:L5   : 240420_0930 サブタスク 1.2

WIP items:
  sample:L10  : 240501_0930 サブタスク 1.3
  sample:L5   : 240420_0930 サブタスク 1.2

TODO items:
  sample:L3   : 240401_0930 タスク 1
```

タイムスタンプとタグを検出してサマリを出力します。
