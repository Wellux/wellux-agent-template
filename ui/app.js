// The cards are not written here. /api/surface sends them.
// Change the look in theme.css. Replace this file if you want another renderer.
// Add a door in crates/shell/src/surface.rs and it shows up on the next load.

const notices = {
  "paypal-captured": "Zahlung erfasst.",
  "paypal-failed": "PayPal hat nicht erfasst.",
  "paypal-cancelled": "Zahlung abgebrochen.",
  "chatgpt-plan": "ChatGPT angemeldet. Plan-Nutzung ist frei.",
  "chatgpt-signed-in": "ChatGPT angemeldet. Plan-Scope fehlt, die Live-API nutzt den API-Key.",
  "chatgpt-failed": "ChatGPT-Anmeldung abgebrochen.",
};

function csrf() {
  const part = document.cookie.split("; ").find((row) => row.startsWith("wellux_csrf="));
  return part ? part.slice("wellux_csrf=".length) : "";
}

async function api(href, body) {
  const response = await fetch(href, {
    method: "POST",
    headers: { "content-type": "application/json", "x-wellux-csrf": csrf() },
    body: JSON.stringify(body || {}),
  });
  const payload = await response.json().catch(() => ({}));
  if (!response.ok) throw new Error(payload.error || "Abgelehnt");
  return payload;
}

function renderDoor(door) {
  const card = document.createElement("section");
  card.className = "door";
  const title = document.createElement("h2");
  title.textContent = door.title;
  const line = document.createElement("p");
  line.textContent = door.line;
  card.append(title, line);
  if (door.missing.length) {
    const miss = document.createElement("p");
    miss.className = "missing";
    miss.textContent = door.connected ? `Hinweis: ${door.missing.join(", ")}` : `Fehlt: ${door.missing.join(", ")}`;
    card.append(miss);
  }
  const form = document.createElement("form");
  for (const field of door.fields) {
    const label = document.createElement("label");
    label.textContent = field.label;
    const input = document.createElement("input");
    input.name = field.name;
    input.type = field.kind;
    input.placeholder = field.placeholder;
    input.autocomplete = field.kind === "password" ? "current-password" : "on";
    label.append(input);
    form.append(label);
  }
  const row = document.createElement("div");
  row.className = "row";
  const error = document.createElement("p");
  error.className = "error";
  for (const action of door.actions) {
    if (action.method === "GET") {
      const link = document.createElement("a");
      link.className = door.connected ? "link" : "link locked";
      link.href = door.connected ? action.href : "#";
      link.textContent = action.label;
      row.append(link);
      continue;
    }
    const button = document.createElement("button");
    button.type = "submit";
    button.className = action.id === door.actions[0].id ? "primary" : "";
    button.textContent = action.label;
    button.disabled = !door.connected && door.id !== "openai";
    button.addEventListener("click", () => {
      button.dataset.href = action.href;
    });
    row.append(button);
  }
  form.append(row, error);
  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    error.textContent = "";
    const href = event.submitter?.dataset.href;
    if (!href) return;
    const body = Object.fromEntries(new FormData(form).entries());
    try {
      const result = await api(href, body);
      if (result.approve) {
        location.href = result.approve;
        return;
      }
      if (result.url) {
        location.href = result.url;
        return;
      }
      if (result.secret) {
        error.textContent = "Live-Secret ist für 10 Minuten in dieser Sitzung. Es wird nicht angezeigt.";
        return;
      }
      await showSession();
    } catch (err) {
      error.textContent = err.message;
    }
  });
  card.append(form);
  return card;
}

async function showSession() {
  const session = await fetch("/api/session").then((r) => r.json());
  const bits = [];
  if (session.supabase?.signed_in) bits.push(`Supabase: ${session.supabase.email}`);
  if (session.chatgpt?.signed_in) bits.push(session.chatgpt.plan ? "ChatGPT mit Plan" : "ChatGPT ohne Plan-Scope");
  document.querySelector("#session").textContent = bits.join(" · ");
}

async function adoptSupabase() {
  if (location.pathname !== "/auth/supabase" || !location.hash.includes("access_token")) return;
  const hash = new URLSearchParams(location.hash.slice(1));
  const accessToken = hash.get("access_token");
  if (!accessToken) return;
  await api("/api/supabase/adopt", { access_token: accessToken, email: "oauth" });
  history.replaceState(null, "", "/");
}

async function main() {
  const params = new URLSearchParams(location.search);
  const notice = notices[params.get("notice")] || "";
  document.querySelector("#notice").textContent = notice;
  await adoptSupabase();
  await showSession();
  const surface = await fetch("/api/surface").then((r) => r.json());
  const brand = document.querySelector("#brand");
  if (surface.project && surface.project !== "wellux-agent-template") {
    brand.innerHTML = "";
    brand.append(document.createTextNode(surface.project + " "));
    const mark = document.createElement("span");
    mark.textContent = "Shell";
    brand.append(mark);
  }
  const repoOk = /^https:\/\/github\.com\/[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/.test(surface.repo || "");
  if (repoOk) {
    const link = document.createElement("a");
    link.href = surface.repo;
    link.textContent = surface.repo.replace("https://github.com/", "");
    link.className = "notice";
    document.querySelector("#notice").prepend(link);
  }
  const root = document.querySelector("#doors");
  root.replaceChildren(...surface.doors.map(renderDoor));
}

main();
