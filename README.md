# 横浜市立図書館の{延滞,予約}ステータスを表示

## 0. 準備

* [横浜市立図書館](https://opac.lib.city.yokohama.lg.jp/winj/opac/top.do)のアカウントを作る

## 1. ステータスを表示

```
$ git clone git@github.com:master-q/jlibnavi.git
$ cd jlibnavi
$ cargo run show USERID PASSWORD
## USERID
延滞資料があります。（1冊）
予約資料が用意できました。（2冊）
```
