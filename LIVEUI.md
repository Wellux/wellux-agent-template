# Live UI

The base package ships in [`ui/`](ui). `make shell` serves it on `127.0.0.1:8787`.

The page does not contain a card for Supabase, PayPal or OpenAI. It asks `/api/surface` and draws whatever that list contains. Connect a door by filling `.env`. The card unlocks. Leave the keys empty and the card says what is missing.

Change the look in `ui/theme.css`. Replace `ui/app.js` if you want another renderer. Add a door in `crates/shell/src/surface.rs` and it appears on the next load without an edit to the HTML.

Tokens never return to the page except the short-lived Realtime secret, which the page keeps in memory and does not print. The ChatGPT refresh token is not stored. The service-role key is never read.

Svelte, Leptos and Tauri are still free to replace `ui/`. They should call the same routes. Do not grow a second policy next to `agent-core`.
