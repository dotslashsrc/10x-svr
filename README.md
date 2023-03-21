# 10x-svr

Defines the server code for the 10x app.

# build and run

From your terminal, do the following:
```sh
cargo b
GPT_API="<the endpoint for a GPT compliant API>" \
GPT_TOKEN="<the token for establishing a secure communication channel with GPT>" \
GPT_DIFF_PROMPT="<the prompt that should be used to properly instruct GPT to analyse diffs>" \
./target/debug/tenx-svr
```
