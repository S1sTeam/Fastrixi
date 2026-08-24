use bytes::Bytes;
use fastrixi_extensions::buffer::BufferExt;
use std::sync::Arc;

use crate::bot::extensions::{BotDefaultExt, BotRotationExt, EntityFilter};
use crate::bot::systems::tasks::{gettskact, killtsk, pushrtsk, TaskName};
use crate::bot::traits::FastrixiModule;
use crate::{sleep, take_bot};

pub struct SwarmGuardOptions {
  pub radius: Option<f64>,
  pub attack_enemies: u8,
  pub state: u8,
}

impl SwarmGuardOptions {
  pub fn from_bytes(buf: &mut Bytes) -> Option<Self> {
    Some(Self {
      radius: Option::read(buf)?,
      attack_enemies: u8::read(buf)?,
      state: u8::read(buf)?,
    })
  }
}

pub struct SwarmGuardModule;

impl SwarmGuardModule {
  pub fn new() -> Self {
    Self
  }

  async fn execute_guard_loop(index: u8, guard_radius: f64, attack_enemies: bool) {
    loop {
      let mut target_enemy = None;

      take_bot!(&index, async |bot| {
        if attack_enemies {
          if let Some(enemy) = bot.find_nearest_entity(&EntityFilter::Player, guard_radius) {
            target_enemy = Some(enemy);
          }
        }
      });

      if let Some(enemy) = target_enemy {
        take_bot!(&index, async |bot| {
          bot.look_at_entity(enemy, true);
          bot.swing_arm();
        });

        sleep!(200);
      } else {
        sleep!(400);
      }
    }
  }
}

impl FastrixiModule<SwarmGuardOptions> for SwarmGuardModule {
  fn new() -> Self {
    Self
  }

  async fn switch(&self, index: u8, options: Arc<SwarmGuardOptions>) -> bool {
    if options.state == 1 && !gettskact(&index, TaskName::Stalker).await {
      let r = options.radius.unwrap_or(12.0);
      let attack = options.attack_enemies == 1;
      let task_handle = tokio::spawn(async move {
        Self::execute_guard_loop(index, r, attack).await;
      });
      pushrtsk(&index, TaskName::Stalker, task_handle).await;
      true
    } else if options.state == 0 && gettskact(&index, TaskName::Stalker).await {
      killtsk(&index, TaskName::Stalker).await;
      true
    } else {
      false
    }
  }
}
