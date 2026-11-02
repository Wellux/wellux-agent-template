//! Small, testable policy every shell should call before a tool runs.
//! Redacts obvious secrets and flags computer-use tools for human approval.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardrailOutcome {
    pub text: String,
    pub approval_required: bool,
    pub findings: Vec<&'static str>,
}

const SENSITIVE_TOOLS: &[&str] = &["screenshot", "mouse", "keyboard", "shell", "sandbox.exec"];

pub fn apply_guardrails(input: &str) -> GuardrailOutcome {
    apply_guardrails_for(input, None)
}

pub fn apply_guardrails_for(input: &str, tool: Option<&str>) -> GuardrailOutcome {
    let mut findings = Vec::new();
    let text = redact(input, &mut findings);
    let approval_required = tool.is_some_and(|name| {
        SENSITIVE_TOOLS
            .iter()
            .any(|tool_name| name.eq_ignore_ascii_case(tool_name))
    });
    if approval_required {
        findings.push("approval");
    }
    GuardrailOutcome {
        text,
        approval_required,
        findings,
    }
}

fn redact(input: &str, findings: &mut Vec<&'static str>) -> String {
    let with_mail = redact_emails(input, findings);
    redact_secrets(&with_mail, findings)
}

fn redact_emails(input: &str, findings: &mut Vec<&'static str>) -> String {
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let mut found = false;
    while i < bytes.len() {
        if bytes[i] == b'@' {
            let start = rewind_email(bytes, i);
            let end = forward_email(bytes, i);
            if start < i && end > i + 1 && looks_like_email(&input[start..end]) {
                out.truncate(start);
                out.push_str("[redacted-email]");
                found = true;
                i = end;
                continue;
            }
        }
        let ch = input[i..].chars().next().unwrap_or('\u{FFFD}');
        out.push(ch);
        i += ch.len_utf8();
    }
    if found {
        findings.push("email");
    }
    out
}

fn looks_like_email(token: &str) -> bool {
    let Some((local, host)) = token.split_once('@') else {
        return false;
    };
    !local.is_empty() && host.contains('.') && !host.starts_with('.') && !host.ends_with('.')
}

fn rewind_email(bytes: &[u8], at: usize) -> usize {
    let mut s = at;
    while s > 0 && is_email_byte(bytes[s - 1]) {
        s -= 1;
    }
    s
}

fn forward_email(bytes: &[u8], at: usize) -> usize {
    let mut e = at + 1;
    while e < bytes.len() && is_email_byte(bytes[e]) {
        e += 1;
    }
    e
}

fn is_email_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'%' | b'+' | b'-')
}

fn redact_secrets(input: &str, findings: &mut Vec<&'static str>) -> String {
    const MARKERS: &[&str] = &["sk-orca-", "sk-", "xai-"];
    let lower = input.to_ascii_lowercase();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let mut found = false;
    let chars: Vec<char> = input.chars().collect();
    let lower_chars: Vec<char> = lower.chars().collect();
    while i < chars.len() {
        let rest: String = lower_chars[i..].iter().collect();
        if let Some(marker) = MARKERS.iter().find(|marker| rest.starts_with(*marker)) {
            let mut j = i + marker.chars().count();
            let mut secret_len = 0;
            while j < chars.len() && is_secret_char(chars[j]) {
                secret_len += 1;
                j += 1;
            }
            if secret_len >= 8 {
                out.push_str("[redacted-secret]");
                found = true;
                i = j;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    if found {
        findings.push("secret");
    }
    out
}

fn is_secret_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '-' | '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_email_and_secret() {
        let outcome = apply_guardrails("ping ada@wellux.dev about sk-abc1234567890");
        assert_eq!(
            outcome.text,
            "ping [redacted-email] about [redacted-secret]"
        );
        assert!(!outcome.approval_required);
        assert!(outcome.findings.contains(&"email"));
        assert!(outcome.findings.contains(&"secret"));
    }

    #[test]
    fn computer_use_needs_a_human() {
        let outcome = apply_guardrails_for("click the button", Some("screenshot"));
        assert!(outcome.approval_required);
        assert!(outcome.findings.contains(&"approval"));
    }

    #[test]
    fn plain_chat_passes_through() {
        let outcome = apply_guardrails("hello core");
        assert_eq!(outcome.text, "hello core");
        assert!(outcome.findings.is_empty());
    }
}
