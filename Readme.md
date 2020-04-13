# Telegram Bot Command Line Interface

This handy tool should allow you to use a telegram bot like a logger.

### Arguments:

`-s` or `--send-only` immediately exits as soon as there is nothing more to send (stdin closes)

`-i` or `--id` sets the id of the person/group/channel you want to send the message to.

Can also be supplied via environment variable: `TELEGRAM_RECEIVER_ID`

`-t` or `--token` sets the token for the bot you are using.

Can also be supplied via environment variable: `TELEGRAM_BOT_TOKEN`

# Examples

__(These assume that you are using the environment variables to set the receiver and bot token!)__

## Sending a message

You can use this to pipe anything into the bot:
- It will send a message per new line.

```bash
% echo "Test" | tgcli --send-only
```

To be informed of any server errors on a development box.

```bash
% tail -f /var/log/nginx/access.log | grep 503 | tgcli --send-only
```

## Receiving messages

Print them to the command line:
- One message per line.
- Ignores non-text messages.

```bash
% tbcli < /dev/null
USERNAME,FIRST_NAME,LAST_NAME,USER_ID,CHAT_ID,EPOCH_SECONDS,MESSAGE_TEXT
```

Ignore all but the message:
```bash
% tbcli < /dev/null | cut -d, -f7
``` 

# Known Bugs

1. In receive-mode, the input and output race with each other, so sending and receiving at the same time is problematic.
   Best to use `< /dev/null` to close the input stream, so the output stream has the lock guaranteed.
