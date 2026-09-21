//! TypeSafe AI (System One Decision Engine) integration for Aizen CLI.
//!
//! TypeSafe's System One models (such as `jev-1.13.0`) evaluate natural language and
//! application state into typed, calibrated judgments and probabilities (`noul`, `choice`, `score`).
//!
//! Subcommands under `aizen typesafe`:
//!   - `test`  — Probe connection and validate TypeSafe API credentials.
//!   - `guard` — Pre-flight safety evaluation for shell commands.
//!   - `route` — Task intent & complexity triage to recommend model tiers.
//!   - `judge` — Flexible query interface for arbitrary System One decisions.

use crate::core::cli_config;
use anyhow::{bail, Context, Result};
use console::{style, Emoji};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

static CHECK: Emoji<'_, '_> = Emoji("✔ ", "[OK] ");
#[allow(dead_code)]
static WARN: Emoji<'_, '_> = Emoji("⚠ ", "[WARN] ");
#[allow(dead_code)]
static CROSS: Emoji<'_, '_> = Emoji("✖ ", "[ERR] ");
static SHIELD: Emoji<'_, '_> = Emoji("🛡 ", "[GUARD] ");
static COMPASS: Emoji<'_, '_> = Emoji("🧭 ", "[ROUTE] ");

pub const DEFAULT_TYPESAFE_ENDPOINT: &str = "https://api.typesafe.ai/v1/systemone";
pub const DEFAULT_TYPESAFE_MODEL: &str = "jev-1.13.0";

// ─── Data Models ─────────────────────────────────────────────────────────────

/// A question specification sent to TypeSafe System One.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Question {
    /// Binary boolean judgment returning probability 0.0 ..= 1.0.
    #[serde(rename = "noul")]
    Noul { instructions: String },
    /// Multi-choice selection picking one option with confidence and probability distribution.
    #[serde(rename = "choice")]
    Choice {
        instructions: String,
        criteria: HashMap<String, String>,
    },
    /// Graded scale rating with probability-weighted score on ordered levels.
    #[serde(rename = "score")]
    Score {
        instructions: String,
        criteria: Vec<String>,
    },
}

/// Request body for `POST /v1/systemone`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeSafeRequest {
    pub model: String,
    pub state: serde_json::Value,
    pub questions: HashMap<String, Question>,
}

/// An answer for a single question from TypeSafe System One.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Answer {
    #[serde(rename = "noul")]
    Noul { noul: f64 },
    #[serde(rename = "choice")]
    Choice {
        choice: String,
        #[serde(default)]
        confidence: f64,
        #[serde(default)]
        probabilities: HashMap<String, f64>,
    },
    #[serde(rename = "score")]
    Score {
        score: f64,
        #[serde(default)]
        confidence: f64,
        #[serde(default)]
        probabilities: HashMap<String, f64>,
        #[serde(default)]
        legend: Option<HashMap<String, String>>,
    },
}

/// Token usage metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Usage {
    #[serde(default)]
    pub input_tokens: u64,
    #[serde(default)]
    pub output_tokens: u64,
}

/// Response payload from `POST /v1/systemone`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeSafeResponse {
    pub model: String,
    pub answers: HashMap<String, Answer>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

// ─── Credential & Endpoint Resolution ────────────────────────────────────────

/// Resolves the TypeSafe API key by checking:
/// 1. Explicit argument (`--api-key`)
/// 2. `TYPESAFE_API_KEY` environment variable
/// 3. Provider profile `jev` or any provider with base URL containing `typesafe.ai` in `cli-config.json`
pub fn resolve_api_key(explicit: Option<&str>) -> Result<String> {
    if let Some(key) = explicit.filter(|s| !s.trim().is_empty()) {
        return Ok(key.trim().to_string());
    }

    if let Ok(env_key) = std::env::var("TYPESAFE_API_KEY") {
        let trimmed = env_key.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    let cfg = cli_config::load();
    if let Some(providers) = &cfg.providers {
        for p in providers {
            let name_lower = p.name.to_lowercase();
            let url_lower = p.base_url.to_lowercase();
            if name_lower == "jev"
                || name_lower.contains("typesafe")
                || url_lower.contains("typesafe.ai")
            {
                let trimmed = p.api_key.trim();
                if !trimmed.is_empty() {
                    return Ok(trimmed.to_string());
                }
            }
        }
    }

    bail!(
        "TypeSafe API key not found.\n\n\
        Resolution paths:\n  \
        1. Provide `--api-key <KEY>`\n  \
        2. Set `export TYPESAFE_API_KEY=<KEY>`\n  \
        3. Save in Aizen config: `aizen config provider add jev --base-url https://api.typesafe.ai/v1/systemone --api-key <KEY> --model jev-1.13.0`"
    )
}

/// Resolves the model to use (`jev-1.13.0` by default).
pub fn resolve_model(explicit: Option<&str>) -> String {
    if let Some(m) = explicit.filter(|s| !s.trim().is_empty()) {
        return m.trim().to_string();
    }
    if let Ok(env_model) = std::env::var("TYPESAFE_MODEL") {
        let trimmed = env_model.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    let cfg = cli_config::load();
    if let Some(providers) = &cfg.providers {
        for p in providers {
            if p.name.eq_ignore_ascii_case("jev") || p.base_url.contains("typesafe.ai") {
                let trimmed = p.model.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }
    }
    DEFAULT_TYPESAFE_MODEL.to_string()
}

/// Resolves the base URL (`https://api.typesafe.ai/v1/systemone` by default).
pub fn resolve_base_url(explicit: Option<&str>) -> String {
    if let Some(u) = explicit.filter(|s| !s.trim().is_empty()) {
        return u.trim().to_string();
    }
    if let Ok(env_url) = std::env::var("TYPESAFE_BASE_URL") {
        let trimmed = env_url.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    let cfg = cli_config::load();
    if let Some(providers) = &cfg.providers {
        for p in providers {
            if p.name.eq_ignore_ascii_case("jev") || p.base_url.contains("typesafe.ai") {
                let trimmed = p.base_url.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }
    }
    DEFAULT_TYPESAFE_ENDPOINT.to_string()
}

// ─── HTTP Client ─────────────────────────────────────────────────────────────

pub struct TypeSafeClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl TypeSafeClient {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            api_key,
            base_url,
            model,
        }
    }

    #[allow(dead_code)]
    pub fn model(&self) -> &str {
        &self.model
    }

    #[allow(dead_code)]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Query TypeSafe System One API.
    pub async fn query(&self, req: &TypeSafeRequest) -> Result<TypeSafeResponse> {
        let resp = self
            .client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(req)
            .send()
            .await
            .with_context(|| {
                format!("Failed to connect to TypeSafe endpoint: {}", self.base_url)
            })?;

        let status = resp.status();
        let body_text = resp
            .text()
            .await
            .unwrap_or_else(|_| "<unreadable response body>".to_string());

        if !status.is_success() {
            bail!(
                "TypeSafe API error (HTTP {}): {}",
                status.as_u16(),
                body_text
            );
        }

        let parsed: TypeSafeResponse = serde_json::from_str(&body_text)
            .with_context(|| format!("Failed to parse TypeSafe JSON response: {}", body_text))?;

        Ok(parsed)
    }
}

// ─── High-Level Command Handlers ─────────────────────────────────────────────

/// `aizen typesafe test`: Connectivity & credential test.
pub async fn handle_test(
    api_key: Option<&str>,
    base_url: Option<&str>,
    model: Option<&str>,
) -> Result<()> {
    let key = resolve_api_key(api_key)?;
    let url = resolve_base_url(base_url);
    let mdl = resolve_model(model);

    println!(
        "{} Probing TypeSafe System One at {} (model: {})...",
        style(SHIELD).cyan(),
        style(&url).yellow(),
        style(&mdl).cyan()
    );

    let client = TypeSafeClient::new(key, url, mdl.clone());

    let mut questions = HashMap::new();
    questions.insert(
        "healthcheck".to_string(),
        Question::Noul {
            instructions: "Is this a connection and credential validation query?".to_string(),
        },
    );

    let req = TypeSafeRequest {
        model: mdl,
        state: serde_json::json!({ "ping": true, "timestamp": chrono::Utc::now().to_rfc3339() }),
        questions,
    };

    let start = Instant::now();
    let resp = client.query(&req).await?;
    let elapsed = start.elapsed();

    println!(
        "{} Connected successfully to TypeSafe System One!",
        style(CHECK).green().bold()
    );
    println!("   Model  : {}", style(resp.model).cyan());
    println!("   Latency: {} ms", style(elapsed.as_millis()).yellow());

    if let Some(usage) = resp.usage {
        println!(
            "   Tokens : {} input, {} output",
            usage.input_tokens, usage.output_tokens
        );
    }

    if let Some(Answer::Noul { noul }) = resp.answers.get("healthcheck") {
        println!(
            "   Probe  : Noul probability = {}",
            style(format!("{:.2}", noul)).green()
        );
    }

    Ok(())
}

/// `aizen typesafe guard <command>`: Pre-flight safety check for shell commands.
pub async fn handle_guard(
    command: &str,
    api_key: Option<&str>,
    base_url: Option<&str>,
    model: Option<&str>,
    json_output: bool,
) -> Result<()> {
    if command.trim().is_empty() {
        bail!("Command cannot be empty. Example: `aizen typesafe guard \"git push --force\"`");
    }

    let key = resolve_api_key(api_key)?;
    let url = resolve_base_url(base_url);
    let mdl = resolve_model(model);
    let client = TypeSafeClient::new(key, url, mdl.clone());

    let mut questions = HashMap::new();
    questions.insert(
        "is_destructive".to_string(),
        Question::Noul {
            instructions: "Does this command permanently delete files, wipe data, drop databases, force push, or cause irreversible changes?".to_string(),
        },
    );

    let mut risk_criteria = HashMap::new();
    risk_criteria.insert(
        "safe".to_string(),
        "Read-only query, inspection, listing, or harmless local check".to_string(),
    );
    risk_criteria.insert(
        "caution".to_string(),
        "Modifies local files, stages commits, installs dependencies, or restarts local services"
            .to_string(),
    );
    risk_criteria.insert(
        "danger".to_string(),
        "Deletes directories, drops tables, force pushes to remote, kills system processes, or modifies system infra".to_string(),
    );
    questions.insert(
        "risk_level".to_string(),
        Question::Choice {
            instructions:
                "Classify the inherent risk level of executing this command in a terminal"
                    .to_string(),
            criteria: risk_criteria,
        },
    );

    let mut action_criteria = HashMap::new();
    action_criteria.insert(
        "allow".to_string(),
        "Execute immediately without prompt".to_string(),
    );
    action_criteria.insert(
        "confirm".to_string(),
        "Require explicit user confirmation before running".to_string(),
    );
    action_criteria.insert(
        "block".to_string(),
        "Block or strongly caution against execution".to_string(),
    );
    questions.insert(
        "policy".to_string(),
        Question::Choice {
            instructions: "Determine the recommended safety policy for this command".to_string(),
            criteria: action_criteria,
        },
    );

    let req = TypeSafeRequest {
        model: mdl,
        state: serde_json::json!({ "command": command }),
        questions,
    };

    let resp = client.query(&req).await?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }

    // Extract values
    let is_destructive = match resp.answers.get("is_destructive") {
        Some(Answer::Noul { noul }) => *noul,
        _ => 0.0,
    };

    let (risk, risk_conf) = match resp.answers.get("risk_level") {
        Some(Answer::Choice {
            choice, confidence, ..
        }) => (choice.as_str(), *confidence),
        _ => ("unknown", 0.0),
    };

    let (policy, policy_conf) = match resp.answers.get("policy") {
        Some(Answer::Choice {
            choice, confidence, ..
        }) => (choice.as_str(), *confidence),
        _ => ("confirm", 0.0),
    };

    println!(
        "\n{} TypeSafe Command Guard Assessment",
        style(SHIELD).bold()
    );
    println!("   Command: {}", style(command).bold().cyan());

    let (badge, badge_color) = match risk {
        "safe" => ("SAFE", console::Color::Green),
        "caution" => ("CAUTION", console::Color::Yellow),
        "danger" => ("DANGER", console::Color::Red),
        _ => ("UNKNOWN", console::Color::White),
    };

    println!(
        "   Risk   : {} (confidence: {:.0}%)",
        style(format!(" [{badge}] ")).bg(badge_color).black().bold(),
        risk_conf * 100.0
    );
    println!(
        "   Destructive probability: {}",
        if is_destructive > 0.5 {
            style(format!("{:.1}%", is_destructive * 100.0))
                .red()
                .bold()
        } else {
            style(format!("{:.1}%", is_destructive * 100.0)).green()
        }
    );
    println!(
        "   Policy : {} (confidence: {:.0}%)\n",
        style(policy.to_uppercase()).bold(),
        policy_conf * 100.0
    );

    Ok(())
}

/// `aizen typesafe route <text>`: Intent & complexity triage for coding tasks.
pub async fn handle_route(
    text: &str,
    api_key: Option<&str>,
    base_url: Option<&str>,
    model: Option<&str>,
    json_output: bool,
) -> Result<()> {
    if text.trim().is_empty() {
        bail!("Task/prompt text cannot be empty. Example: `aizen typesafe route \"Refactor auth module\"`");
    }

    let key = resolve_api_key(api_key)?;
    let url = resolve_base_url(base_url);
    let mdl = resolve_model(model);
    let client = TypeSafeClient::new(key, url, mdl.clone());

    let mut questions = HashMap::new();

    let mut cat_criteria = HashMap::new();
    cat_criteria.insert(
        "coding".to_string(),
        "Writing, editing, or implementing code".to_string(),
    );
    cat_criteria.insert(
        "debugging".to_string(),
        "Troubleshooting errors, logs, or failing tests".to_string(),
    );
    cat_criteria.insert(
        "explanation".to_string(),
        "Explaining concepts, answering questions, or documentation".to_string(),
    );
    cat_criteria.insert(
        "refactor".to_string(),
        "Restructuring architecture or refactoring existing code".to_string(),
    );
    cat_criteria.insert(
        "trivial".to_string(),
        "Short greeting, casual conversation, or simple one-liner".to_string(),
    );
    questions.insert(
        "category".to_string(),
        Question::Choice {
            instructions: "Classify the primary technical category of this request".to_string(),
            criteria: cat_criteria,
        },
    );

    questions.insert(
        "complexity".to_string(),
        Question::Score {
            instructions: "Rate the technical complexity of fulfilling this task on a 1-5 scale"
                .to_string(),
            criteria: vec![
                "Trivial 1-step response".to_string(),
                "Straightforward single-file task".to_string(),
                "Moderate multi-step implementation".to_string(),
                "Complex cross-module architecture or deep debugging".to_string(),
                "Massive enterprise system redesign".to_string(),
            ],
        },
    );

    questions.insert(
        "needs_reasoning".to_string(),
        Question::Noul {
            instructions: "Does this task require deep algorithmic reasoning, complex multi-step planning, or an advanced high-tier model?".to_string(),
        },
    );

    let req = TypeSafeRequest {
        model: mdl,
        state: serde_json::json!({ "task": text }),
        questions,
    };

    let resp = client.query(&req).await?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }

    let category = match resp.answers.get("category") {
        Some(Answer::Choice {
            choice, confidence, ..
        }) => format!("{} ({:.0}%)", choice, confidence * 100.0),
        _ => "unknown".to_string(),
    };

    let complexity = match resp.answers.get("complexity") {
        Some(Answer::Score { score, .. }) => format!("{:.1} / 5.0", score),
        _ => "N/A".to_string(),
    };

    let needs_reasoning = match resp.answers.get("needs_reasoning") {
        Some(Answer::Noul { noul }) => *noul > 0.5,
        _ => false,
    };

    println!(
        "\n{} TypeSafe Task Routing & Complexity Analysis",
        style(COMPASS).bold()
    );
    println!("   Task      : {}", style(text).bold().cyan());
    println!("   Category  : {}", style(category).yellow());
    println!("   Complexity: {}", style(complexity).magenta().bold());
    println!(
        "   Model Tier: {}",
        if needs_reasoning {
            style("Deep Reasoning Tier (e.g. Nemotron-Ultra / Claude-Opus / DeepSeek-R1)")
                .red()
                .bold()
        } else {
            style("Fast / Standard Tier (e.g. Nemotron-3.5 / OxAlpha / GPT-4o-mini)")
                .green()
                .bold()
        }
    );
    println!();

    Ok(())
}

/// `aizen typesafe judge`: Arbitrary System One decision query.
pub async fn handle_judge(
    state: Option<&str>,
    noul: Option<&str>,
    choice: Option<&str>,
    score: Option<&str>,
    raw_json: Option<&str>,
    file: Option<&str>,
    api_key: Option<&str>,
    base_url: Option<&str>,
    model: Option<&str>,
    json_output: bool,
) -> Result<()> {
    let key = resolve_api_key(api_key)?;
    let url = resolve_base_url(base_url);
    let mdl = resolve_model(model);
    let client = TypeSafeClient::new(key, url, mdl.clone());

    let req: TypeSafeRequest = if let Some(path) = file {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read request file at {}", path))?;
        serde_json::from_str(&content).with_context(|| "Failed to parse JSON request from file")?
    } else if let Some(raw) = raw_json {
        serde_json::from_str(raw).with_context(|| "Failed to parse raw JSON request")?
    } else {
        let state_val = if let Some(s) = state {
            serde_json::from_str::<serde_json::Value>(s)
                .unwrap_or_else(|_| serde_json::Value::String(s.to_string()))
        } else {
            serde_json::Value::Null
        };

        let mut questions = HashMap::new();

        if let Some(n) = noul {
            // Supports either "id:instruction" or just "instruction"
            let (qid, inst) = match n.split_once(':') {
                Some((id, i)) if !id.trim().is_empty() => {
                    (id.trim().to_string(), i.trim().to_string())
                }
                _ => ("noul_q".to_string(), n.trim().to_string()),
            };
            questions.insert(qid, Question::Noul { instructions: inst });
        }

        if let Some(c) = choice {
            // Format: "id:instructions:opt1=desc1,opt2=desc2" or "instructions:opt1=desc1,opt2=desc2"
            let parts: Vec<&str> = c.split(':').collect();
            let (qid, inst, opt_str) = match parts.len() {
                3 => (
                    parts[0].trim().to_string(),
                    parts[1].trim().to_string(),
                    parts[2].trim(),
                ),
                2 => (
                    "choice_q".to_string(),
                    parts[0].trim().to_string(),
                    parts[1].trim(),
                ),
                _ => bail!(
                    "Invalid choice format. Expected: `id:instructions:opt1=desc1,opt2=desc2`"
                ),
            };

            let mut criteria = HashMap::new();
            for item in opt_str.split(',') {
                if let Some((k, v)) = item.split_once('=') {
                    criteria.insert(k.trim().to_string(), v.trim().to_string());
                } else if !item.trim().is_empty() {
                    criteria.insert(item.trim().to_string(), item.trim().to_string());
                }
            }
            questions.insert(
                qid,
                Question::Choice {
                    instructions: inst,
                    criteria,
                },
            );
        }

        if let Some(s) = score {
            // Format: "id:instructions:lvl0,lvl1,lvl2..." or "instructions:lvl0,lvl1,lvl2..."
            let parts: Vec<&str> = s.split(':').collect();
            let (qid, inst, levels_str) = match parts.len() {
                3 => (
                    parts[0].trim().to_string(),
                    parts[1].trim().to_string(),
                    parts[2].trim(),
                ),
                2 => (
                    "score_q".to_string(),
                    parts[0].trim().to_string(),
                    parts[1].trim(),
                ),
                _ => bail!("Invalid score format. Expected: `id:instructions:lvl0,lvl1,lvl2`"),
            };
            let criteria: Vec<String> = levels_str
                .split(',')
                .map(|item| item.trim().to_string())
                .filter(|item| !item.is_empty())
                .collect();
            questions.insert(
                qid,
                Question::Score {
                    instructions: inst,
                    criteria,
                },
            );
        }

        if questions.is_empty() {
            bail!("No questions provided. Specify at least one of `--noul`, `--choice`, `--score`, or `--raw`.");
        }

        TypeSafeRequest {
            model: mdl,
            state: state_val,
            questions,
        }
    };

    let resp = client.query(&req).await?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&resp)?);
    } else {
        println!(
            "\n{} TypeSafe System One Decision Result",
            style(CHECK).green().bold()
        );
        println!("   Model: {}", style(resp.model).cyan());
        for (qid, ans) in &resp.answers {
            match ans {
                Answer::Noul { noul } => {
                    println!(
                        "   {} {}: probability = {} (Answer: {})",
                        style("[NOUL]").bold().blue(),
                        style(qid).bold(),
                        style(format!("{:.2}", noul)).yellow(),
                        if *noul >= 0.5 {
                            style("YES").green().bold()
                        } else {
                            style("NO").red().bold()
                        }
                    );
                }
                Answer::Choice {
                    choice,
                    confidence,
                    probabilities,
                } => {
                    println!(
                        "   {} {}: selected '{}' (confidence: {:.0}%)",
                        style("[CHOICE]").bold().magenta(),
                        style(qid).bold(),
                        style(choice).green().bold(),
                        confidence * 100.0
                    );
                    if !probabilities.is_empty() {
                        let mut sorted: Vec<_> = probabilities.iter().collect();
                        sorted.sort_by(|a, b| {
                            b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal)
                        });
                        let dist: Vec<String> = sorted
                            .iter()
                            .map(|(k, v)| format!("{}: {:.0}%", k, *v * 100.0))
                            .collect();
                        println!("            distribution: {}", dist.join(", "));
                    }
                }
                Answer::Score {
                    score, confidence, ..
                } => {
                    println!(
                        "   {} {}: score = {} (confidence: {:.0}%)",
                        style("[SCORE]").bold().yellow(),
                        style(qid).bold(),
                        style(format!("{:.2}", score)).green().bold(),
                        confidence * 100.0
                    );
                }
            }
        }
        if let Some(usage) = resp.usage {
            println!(
                "   Tokens: {} input, {} output\n",
                usage.input_tokens, usage.output_tokens
            );
        } else {
            println!();
        }
    }

    Ok(())
}

// ─── Unit Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_serialization_noul() {
        let q = Question::Noul {
            instructions: "Is this dangerous?".to_string(),
        };
        let val = serde_json::to_value(&q).unwrap();
        assert_eq!(val["type"], "noul");
        assert_eq!(val["instructions"], "Is this dangerous?");
    }

    #[test]
    fn test_question_serialization_choice() {
        let mut criteria = HashMap::new();
        criteria.insert("safe".to_string(), "No risk".to_string());
        let q = Question::Choice {
            instructions: "Pick one".to_string(),
            criteria,
        };
        let val = serde_json::to_value(&q).unwrap();
        assert_eq!(val["type"], "choice");
        assert_eq!(val["criteria"]["safe"], "No risk");
    }

    #[test]
    fn test_question_serialization_score() {
        let q = Question::Score {
            instructions: "Rank 1-3".to_string(),
            criteria: vec!["low".to_string(), "med".to_string(), "high".to_string()],
        };
        let val = serde_json::to_value(&q).unwrap();
        assert_eq!(val["type"], "score");
        assert_eq!(val["criteria"][0], "low");
        assert_eq!(val["criteria"][2], "high");
    }

    #[test]
    fn test_answer_deserialization() {
        let noul_json = r#"{"type": "noul", "noul": 0.88}"#;
        let ans: Answer = serde_json::from_str(noul_json).unwrap();
        match ans {
            Answer::Noul { noul } => assert!((noul - 0.88).abs() < 1e-6),
            _ => panic!("Expected Noul answer"),
        }

        let choice_json = r#"{"type": "choice", "choice": "danger", "confidence": 0.95, "probabilities": {"safe": 0.05, "danger": 0.95}}"#;
        let ans2: Answer = serde_json::from_str(choice_json).unwrap();
        match ans2 {
            Answer::Choice {
                choice, confidence, ..
            } => {
                assert_eq!(choice, "danger");
                assert!((confidence - 0.95).abs() < 1e-6);
            }
            _ => panic!("Expected Choice answer"),
        }
    }
}
