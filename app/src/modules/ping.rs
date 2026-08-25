use std::net::{IpAddr, Ipv4Addr};
use std::time::Instant;

use dns_lookup::lookup_host;
use serde::Serialize;

#[derive(Serialize)]
pub struct Player {
  username: String,
  uuid: String,
}

#[derive(Serialize)]
pub struct ServerInformation {
  pub ip_address: String,
  pub port: u16,
  pub latency_ms: u64,
  pub server_icon: Option<String>,
  pub protocol_version: i32,
  pub server_version: String,
  pub server_core: String,
  pub protection: String,
  pub description: String,
  pub players_online: i32,
  pub players_max: i32,
  pub list_of_players: Vec<Player>,
  pub success: bool,
}

/// Функция пингования сервера с реальным измерением задержки, резолвом и анализом ядра/защиты
pub async fn ping_server(address: String) -> ServerInformation {
  let split_address: Vec<&str> = address.split(':').collect();
  let host_part = split_address.get(0).copied().unwrap_or("").trim();
  let port_part = split_address
    .get(1)
    .and_then(|p| p.parse::<u16>().ok())
    .unwrap_or(25565);

  let mut resolved_ip = host_part.to_string();
  if let Ok(resp) = lookup_host(host_part) {
    let ips: Vec<IpAddr> = resp.collect();
    if let Some(ip) = ips.get(0) {
      resolved_ip = ip.to_string();
    }
  }

  let mut info = ServerInformation {
    ip_address: resolved_ip,
    port: port_part,
    latency_ms: 0,
    server_icon: None,
    protocol_version: -1,
    server_version: "Неизвестно".to_string(),
    server_core: "Офлайн / Недоступен".to_string(),
    protection: "Не удалось подключиться".to_string(),
    description: "".to_string(),
    players_online: 0,
    players_max: 0,
    list_of_players: Vec::new(),
    success: false,
  };

  if host_part.is_empty() {
    return info;
  }

  let start = Instant::now();
  let ping_response = azalea::ping::ping_server(address).await;
  let elapsed = start.elapsed().as_millis() as u64;

  match ping_response {
    Ok(resp) => {
      info.success = true;
      info.latency_ms = elapsed;
      info.server_icon = resp.favicon;
      info.protocol_version = resp.version.protocol;
      info.server_version = resp.version.name.clone();
      info.description = resp.description.to_html();
      info.players_online = resp.players.online;
      info.players_max = resp.players.max;

      let v_name = resp.version.name.trim();
      let desc_plain = resp.description.to_string().to_lowercase();
      let lower_v = v_name.to_lowercase();

      // Детект реального ядра сервера
      if lower_v.contains("velocity") {
        info.server_core = format!("Velocity Proxy ({})", v_name);
      } else if lower_v.contains("paper") {
        info.server_core = format!("Paper ({})", v_name);
      } else if lower_v.contains("purpur") {
        info.server_core = format!("Purpur ({})", v_name);
      } else if lower_v.contains("spigot") {
        info.server_core = format!("Spigot ({})", v_name);
      } else if lower_v.contains("bungee") {
        info.server_core = format!("BungeeCord ({})", v_name);
      } else if lower_v.contains("waterfall") {
        info.server_core = format!("Waterfall ({})", v_name);
      } else if lower_v.contains("fabric") {
        info.server_core = format!("Fabric ({})", v_name);
      } else if lower_v.contains("forge") {
        info.server_core = format!("Forge ({})", v_name);
      } else if !v_name.is_empty() {
        info.server_core = format!("Minecraft Server ({})", v_name);
      } else {
        info.server_core = "Minecraft Server".to_string();
      }

      // Детект активных защит и фильтров
      let mut detected = Vec::new();
      if lower_v.contains("nullcord") || desc_plain.contains("nullcord") {
        detected.push("NullCordX");
      }
      if lower_v.contains("flamecord") || desc_plain.contains("flamecord") {
        detected.push("FlameCord");
      }
      if lower_v.contains("limbo") || desc_plain.contains("limbo") {
        detected.push("LimboAPI / LimboFilter");
      }
      if lower_v.contains("bungeeguard") || desc_plain.contains("bungeeguard") {
        detected.push("BungeeGuard");
      }
      if lower_v.contains("tcpshield") || desc_plain.contains("tcpshield") {
        detected.push("TCPShield");
      }
      if lower_v.contains("antibot") || desc_plain.contains("antibot") || desc_plain.contains("botfilter") {
        detected.push("AntiBot Filter");
      }

      if !detected.is_empty() {
        info.protection = format!("{} (Обнаружено)", detected.join(" + "));
      } else {
        info.protection = "Не обнаружено (Стандартный)".to_string();
      }

      for player in resp.players.sample {
        info.list_of_players.push(Player {
          username: player.name,
          uuid: player.id,
        });
      }
    }
    Err(e) => {
      info.success = false;
      info.server_core = "Офлайн / Ошибка соединения".to_string();
      info.protection = format!("Не удалось пингануть: {}", e);
    }
  }

  info
}

#[tauri::command]
pub async fn get_server_info(address: String) -> ServerInformation {
  ping_server(address).await
}
