# Access

Three doors. None of them share a secret with the browser except a publishable id or a secret that dies with the session.

`agent_core::access` is the checklist and the door functions. They do not call the network. A shell fills in the HTTP.

| Function | Door |
|----------|------|
| `pkce_s256` | Supabase OAuth and ChatGPT. S256, RFC 7636 vector is tested. |
| `redirect_allowed` | Exact redirect. A `*` in the allow-list fails closed. |
| `paypal_order` | Amount in minor units. Zero is rejected. `body()` is the Orders API JSON. |
| `chatgpt_authorize_url` | PKCE authorize URL. `dynamic_agent_client` only on the first registration. |
| `plan_usage_granted` | True only when `chatgpt.tokens.use.direct` is its own scope. |
| `browser_realtime_secret` | Accepts the ephemeral secret. Rejects an `sk-` or `xai-` key. |

## Supabase — user login and OAuth

Use Supabase Auth for email and for OAuth providers you explicitly enable (Google, GitHub, Apple). Flow is PKCE.

| Name | Where it lives |
|------|----------------|
| `SUPABASE_URL` | Browser |
| `SUPABASE_PUBLISHABLE_KEY` | Browser |
| `SUPABASE_SERVICE_ROLE_KEY` | Server only |

Rules:

- Allowlist redirect URLs. No open wildcard.
- Turn on Row Level Security for every table.
- The client never sends a user id as proof. The session token is the proof.
- Identify the user by the `sub` claim of a token you verified, not by an email typed into a form.
- The service-role key bypasses RLS. It does not belong in a mobile app, a Tauri webview bundle, or a log line.

## PayPal — checkout

Use the current Orders API. Do not use the retired client-only `checkout.js` flow.

| Name | Where it lives |
|------|----------------|
| `PAYPAL_CLIENT_ID` | Browser, inside the JS SDK button |
| `PAYPAL_CLIENT_SECRET` | Server only |
| `PAYPAL_WEBHOOK_ID` | Server only |
| `PAYPAL_ENV` | `sandbox` until capture and the webhook are proven |

Server:

1. Exchange the client id and secret for an access token at `https://api-m.sandbox.paypal.com` (live host: `https://api-m.paypal.com`).
2. `POST /v2/checkout/orders` with an amount **your server** chose.
3. Give the browser the order id. The button approves it.
4. `POST /v2/checkout/orders/{id}/capture` on the server.
5. Verify the webhook signature before you unlock anything.

A click is not a payment. A verified capture is.

## OpenAI — ChatGPT plan and the live API

These are different credentials. Signing in does not hand you an API key.

### Continue with ChatGPT

Desktop and agent shells use the open-source Sign in with ChatGPT flow:

- Authorize: `https://auth.openai.com/api/accounts/authorize`
- Token: `https://auth.openai.com/api/accounts/oauth/token`
- Authorization code + PKCE (`S256`). No client secret.
- Validate `state` on return and `nonce` inside the ID token.
- Verify the ID token against OpenAI's JWKS. Check issuer, audience, expiry.
- First registration may use `client_id=dynamic_agent_client` plus your app name. Persist the **issued** client id (`oaiapp_…`). Do not store `dynamic_agent_client` as if it were yours.
- Scopes: `openid profile email offline_access resource.invoke chatgpt.tokens.use.direct`.
- Plan usage is allowed only when the token response actually includes `chatgpt.tokens.use.direct`. An ID token alone is not that permission.
- Redirect for a local agent is a loopback URI (`http://127.0.0.1:<port>/auth/callback`) that matches exactly. A website uses the redirect you registered, then keeps the refresh token in an httpOnly cookie or the OS keychain — never in `localStorage`, never in git.

The refresh token is `CHATGPT_REFRESH_TOKEN` in `access::boundaries`: keychain, mode `0600` if it is a file. Do not log it.

### Live / Realtime API

The browser must not hold `OPENAI_API_KEY`.

1. Your server calls `POST https://api.openai.com/v1/realtime/client_secrets` with the server credential.
2. The browser receives only that ephemeral secret.
3. The browser opens the live session over WebRTC at `/v1/realtime/calls`.
4. A server-side worker may use a WebSocket instead. That socket still starts on the server.

Use the ChatGPT access token as that server credential only when the granted scope includes API invocation. Otherwise use `OPENAI_API_KEY` and bill the platform account. Do not point a ChatGPT password, session cookie, or refresh token at the browser SDK.

OrcaRouter remains the default gateway for ordinary chat completions (`https://api.orcarouter.ai/v1`). Realtime stays on OpenAI's own live endpoint.

## Setup order

```bash
cp .env.example .env
make doctor
```

Then fill only the doors you need. `make doctor` warns if `.env` is missing and if git is tracking it.
