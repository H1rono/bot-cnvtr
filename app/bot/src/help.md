# BOT_cnvtr

いろんなWebhookを受け取るtraQ BOTです。開発中です。

## 使い方

1. Webhookを送信したいチャンネルで`@BOT_cnvtr webhook create`を投稿します
    - 色々とカスタマイズすることもできます、詳しくは`@BOT_cnvtr webhook --help`
2. Webhookが作成され、DMにその情報が送られます。
3. 各対応サービスに作成されたWebhookを登録します。

## 対応サービス

Webhookが現在対応しているサービス一覧は以下の通りです。

### GitHub

登録方法は[Creating webhooks - GitHub Docs](https://docs.github.com/en/webhooks/using-webhooks/creating-webhooks)を参考にしてください。Content typeは`application/json`にのみ対応しています。Organization Webhookにも対応していますが、Repository Webhookでの使用を想定しています。

### Gitea

以下の手順でWebhookを登録することができます。

1. Webhookを登録したいGiteaのリポジトリを開く
2. 設定 > Webhook
3. Webhookを追加 > Gitea
4. 表示されるフォームに適切な値を入力
    - ターゲットURLはDMで送られたもの
    - HTTPメソッドはPOST
    - POST Content Typeは`application/json`

### Clickup

(開発中) :construction:

## Contributing

バグ報告は:@H1rono_K:まで。Pull Requestも大歓迎です
リポジトリ: [H1rono/bot-cnvtr](https://github.com/H1rono/bot-cnvtr)
