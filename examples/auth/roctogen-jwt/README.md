# GitHub webassembly JWT authentication example

This example runs on a [Cloudflare webassembly
worker](https://cloudflareworkers.com/), and authenticates itself to GitHub
using an [existing GitHub
App](https://docs.github.com/en/developers/apps/authenticating-with-github-apps#authenticating-as-a-github-app)
and associated private key. 

Once the application creates a JWT token, it fetches a list of installations
for the associated GitHub App and uses the first installation id to [create an
installation access
token](https://docs.github.com/en/rest/reference/apps#create-an-installation-access-token-for-an-app)
for the API endpoints that the GitHub App installation has given access to.

The resulting token could then be used to access GitHub endpoints that require
token-level access.

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
