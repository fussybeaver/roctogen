# GitHub webassembly JWT authentication example

This example runs on a [Cloudflare webassembly
worker](https://cloudflareworkers.com/), and authenticates itself to GitHub
using an [existing GitHub
App](https://docs.github.com/en/developers/apps/authenticating-with-github-apps#authenticating-as-a-github-app)
and associated private key. 

## Requirements

These instructions assume an existing Cloudflare account and a GitHub App with
an associated private key.

Ensure the `wrangler` command-line application is available.

## Configuration

Copy the `wrangler.toml.example` to `wrangler.toml` and fill the `account_id`
field with your associated account ID.

## Secrets

Two secrets are stored in Cloudflare:

  - RSA Private Key
  - GitHub App ID

In order to work around the [1KB
limit](https://developers.cloudflare.com/workers/platform/limits) of the
Cloudflare free tier, split the private key and re-assemble it at runtime:

```bash
split -b 1024 key.pem
```

Put the secrets into appropriate keys:

```bash
wrangler secret put MY_APPLICATION_KEY_PART1 < xaa
wrangler secret put MY_APPLICATION_KEY_PART2 < xab
```

Put the GitHub App ID into a secret:

```bash
echo 111111 | wrangler secret put MY_APPLICATION_APP_ID
```

## Javascript

Edit the [`worker/worker.js`](./worker/worker.js) appropriately to fetch
secrets and re-assemble keys appropriately.
