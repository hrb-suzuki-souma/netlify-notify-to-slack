コンソールやCIで `netlify deploy` した時に発行されるDeployURLをSlackに通知するCIツールです

```bash
# 事前にIncomingWebhookのURLを発行してください
$ export SLACK_WEBHOOK_URL=<YOUR_WEBHOOK_URL>
$ netlify deploy --json | netlify-to-slack
```
