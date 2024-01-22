
# Deft Reversi Engine

<img src="https://img.shields.io/badge/-Rust-000000.svg?logo=rust&style=plastic">

Rustで書かれたオセロAI。

## 使用した技術
 - bitboard
 - negascout探索 (PVS)
 - 置換表
 - 機械学習(線形回帰)を用いた評価関数

## Deft Reversi Web
Deft Reversi Engineを搭載したオセロゲームは、以下のウェブサイトでプレイできます:

[ Deft Reversi](https://az.recazbowl.net/deft_web/)

[ Deft Reversi (Github)](https://github.com/ikepggthb/deft_web)

作者は、同等のAIをCodeinGameで使用しています。(こちらは、機械学習した評価関数を用いていません。)
現在、世界17位となっています。
https://www.codingame.com/multiplayer/bot-programming/othello-1/leaderboard

## ライセンス
このプロジェクトは[MIT License](https://opensource.org/license/mit/)の下で公開されています。
