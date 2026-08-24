use azalea::protocol::packets::game::ServerboundAcceptTeleportation;
use azalea::Client;
use std::time::Duration;
use tokio::time::sleep;

use crate::bot::extensions::BotPhysicsExt;

/// Движок эмуляции физики падения в пустоте для обхода LimboFilter и LimboAPI
pub struct LimboPhysicsEngine;

impl LimboPhysicsEngine {
  /// Точная симуляция тика физики гравитации Minecraft:
  /// v_y = (v_y - 0.08) * 0.98
  pub fn calculate_next_fall_velocity(current_vy: f64) -> f64 {
    (current_vy - 0.08) * 0.98
  }

  /// Обработка телепортации в LimboWorld с мгновенным подтверждением
  pub fn handle_teleport(bot: &Client, teleport_id: u32) {
    bot.write_packet(ServerboundAcceptTeleportation {
      id: teleport_id,
    });
  }

  /// Цикл симуляции гравитации падения в лобби Limbo (20 TPS)
  pub async fn simulate_limbo_fall(bot: Client, ticks: u32) {
    let mut current_vy = 0.0;

    for _ in 0..ticks {
      current_vy = Self::calculate_next_fall_velocity(current_vy);
      bot.set_velocity("y", current_vy);
      sleep(Duration::from_millis(50)).await;
    }
  }
}
