# AWK
![CI Status](https://github.com/kat0h/rusty_awk/actions/workflows/ci.yml/badge.svg)

POSIXで規定されたAWKの実装です。  
[POSIXの2017年版](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/awk.html)を参照していますが、一部異なる仕様があります。

- 数字と文字列を比較した場合、数値は文字列に変換されて比較されます
    - POSIXの仕様は誤っています。nawk/gawkと同じ挙動です

## ビルド方法
Rust言語で実装されています。cargoコマンドが使えるようにしてください。

`$ cargo build --release`でプログラムがビルドされます。

## 実装されている機能

- BEGIN/END/式によるパターン
  - `$ rusty_awk 'BEGIN{print 123}'` => 123
    - BEGINパターンはプログラムの最初に実行されます
  - `$ echo OK | rusty_awk '1==1'` => OK
    - 条件式によるパターンの実行が可能です
  - `$ seq 10 | rusty_awk '$1%2==0{print $1 "は偶数です"};$1%2{print $1 "は奇数です"}'` => 1は奇数です, 2は偶数です...
    - 1~10までの数字が奇数であるか、偶数であるかを判定するサンプルです
  
- 変数
  - `$ rusty_awk 'BEGIN{a=1;print a;a+=1;print a}'` => 1, 2
    - 変数の動作です
  - `$ seq 10 | rusty_awk 'BEGIN{a=0};{a+=$1};END{print "合計:" a}'` => 合計:55
    - 1~10までの数値を合計するサンプルです

## 演算子
  - (expr)
    - 括弧
  - $expr
    - フィールドへのアクセス
  - ++lvalue
  - -- lvalue
  - lvalue++
  - lvalue--
    - {前置,後置}{インクリメント,デクリメント}
  - expr ^ expr
    - 累乗
  - !expr
    - 論理否定
  - +expr
    - 明示的なプラス
  - -expr
    - マイナス記号
  - expr * expr
  - expr / expr
  - expr % expr
  - expr + expr
  - expr - expr
    - 四則演算
  - expr expr
    - 文字列の結合
  - expr < expr
  - expr <= expr
  - expr != expr
  - expr == expr
  - expr > expr
  - expr >= expr
    - 比較
  - expr && expr
  - expr || expr
    - 論理演算
  - lvalue ^= expr
  - lvalue %= expr
  - lvalue *= expr
  - lvalue /= expr
  - lvalue += expr
  - lvalue -= expr
  - evalue = expr
    - 代入

## 動作
引数に与えられたAWKプログラムは再帰下降構文解析パーサーによって構文木に変換されます。
変換されたプラグラムは実行機で解釈されます。

## ライセンス
MIT

## 著者
kota kato
