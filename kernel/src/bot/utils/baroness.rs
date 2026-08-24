use regex::Regex;
use serde_json::Value;

/// Модуль парсинга и решения интерактивных кликабельных компонентов BaronessAuth и чат-капчи
pub struct BaronessAuthSolver;

impl BaronessAuthSolver {
  /// Парсинг JSON-компонента чата на наличие события клика (`clickEvent`)
  pub fn extract_click_command(chat_json: &str) -> Option<String> {
    if let Ok(value) = serde_json::from_str::<Value>(chat_json) {
      if let Some(cmd) = Self::find_click_event(&value) {
        return Some(cmd);
      }
    }

    // Резервный поиск через регулярное выражение, если сервер отправляет текстовый промпт
    let regex = Regex::new(r"/(?:captcha|code|verify|auth|login)\s+([a-zA-Z0-9]{4,8})").ok()?;
    if let Some(captures) = regex.captures(chat_json) {
      if let Some(matched) = captures.get(0) {
        return Some(matched.as_str().to_string());
      }
    }

    None
  }

  /// Рекурсивный поиск clickEvent внутри JSON-дерева чата Minecraft
  fn find_click_event(val: &Value) -> Option<String> {
    if let Value::Object(map) = val {
      if let Some(click_event) = map.get("clickEvent") {
        if let Some(action) = click_event.get("action").and_then(|a| a.as_str()) {
          if action == "run_command" || action == "suggest_command" {
            if let Some(val_cmd) = click_event.get("value").and_then(|v| v.as_str()) {
              return Some(val_cmd.to_string());
            }
          }
        }
      }

      if let Some(extra) = map.get("extra").and_then(|e| e.as_array()) {
        for child in extra {
          if let Some(cmd) = Self::find_click_event(child) {
            return Some(cmd);
          }
        }
      }
    } else if let Value::Array(arr) = val {
      for item in arr {
        if let Some(cmd) = Self::find_click_event(item) {
          return Some(cmd);
        }
      }
    }

    None
  }
}
