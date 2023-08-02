# Introduction

vonage-to-matrix is a simple tool that on the *input* handles [Vonage WebHook](https://developer.vonage.com/en/messaging/sms/guides/inbound-sms) data about incoming calls and SMS messages and on the *output* posts that data to the [Matrix Hookshot WebHook](https://github.com/matrix-org/matrix-hookshot).

# Configuration
The service can be configured by editing the `config.toml` file.
## Hookshot webhook URL
Place the webhook URL in the *hookshot* section like so:
```
[hookshot]
url = "https://hookshot.teonite.net/webhook/141d7f48-862e-42a8-a1e7-efab3a37ed09"
```
# Configuring message and call forwarding in Vonage
1. Sign in to vonage and add go to [your numbers](https://dashboard.nexmo.com/your-numbers).
2. Open configuration panel for a number by clicking the edit button.
3. Place the following URL under *SMS* -> *Inbound Webhook URL* to enable message forwarding
   ```
   https://vonage.teonite.net/api/inbound-message
   ```
4. Enable call forwarding by picking *Phone* in the *Forward to* menu and typing in the *Number*. Place the following URL in *Event Webhook URL*
   ```
   https://vonage.teonite.net/api/inbound-call
   ```
# Number labeling
The service can display labeled numbers in messages forwarded to a webhook. To add label to a number one needs to edit the config.toml. Labels should be places under *vonage.labels* key like in the following file:
```
[vonage.labels]
48111111111 = "Label for this number"
12222222222 = "Different label"
```