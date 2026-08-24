use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use azalea::{BlockPos, Vec3};
use bytes::Bytes;
use fastrixi_extensions::buffer::BufferExt;
use std::sync::Arc;

use crate::bot::extensions::{BotDefaultExt, BotInteractExt, BotInventoryExt, BotRotationExt, EntityFilter};
use crate::bot::systems::states::{getst, State};
use crate::bot::systems::tasks::{gettskact, killtsk, pushrtsk, TaskName};
use crate::bot::traits::FastrixiModule;
use crate::{sleep, take_bot};

pub struct AutoCrystalOptions {
  pub distance: Option<f64>,
  pub max_self_damage: Option<f32>,
  pub state: u8,
}

impl AutoCrystalOptions {
  pub fn from_bytes(buf: &mut Bytes) -> Option<Self> {
    Some(Self {
      distance: Option::read(buf)?,
      max_self_damage: Option::read(buf)?,
      state: u8::read(buf)?,
    })
  }
}

pub struct AutoCrystalModule;

impl AutoCrystalModule {
  pub fn new() -> Self {
    Self
  }

  async fn execute_crystal_loop(index: u8, target_distance: f64, max_self_damage: f32) {
    loop {
      if !getst(&index, State::CanAttacking).await {
        sleep!(50);
        continue;
      }

      let mut enemy_pos = None;
      let mut self_pos = None;

      take_bot!(&index, async |bot| {
        if let Some(target) = bot.find_nearest_entity(&EntityFilter::Player, target_distance) {
          enemy_pos = Some(bot.get_entity_position(target));
        }
        self_pos = bot.foot_pos();
      });

      if let (Some(enemy), Some(my_pos)) = (enemy_pos, self_pos) {
        let target_block = BlockPos::new(enemy.x.floor() as i32, (enemy.y.floor() - 1.0) as i32, enemy.z.floor() as i32);
        let dist_to_me = my_pos.distance_to(Vec3::new(target_block.x as f64, target_block.y as f64, target_block.z as f64));

        if dist_to_me < 2.0 && max_self_damage < 8.0 {
          sleep!(100);
          continue;
        }

        take_bot!(&index, async |bot| {
          if let Some(obs_slot) = bot.inventory_find_item(ItemKind::Obsidian) {
            bot.inventory_move_item(&index, ItemKind::Obsidian, obs_slot, 36, false).await;
            bot.look_at_block(target_block, false).await;
            bot.start_use_item_by(InteractionHand::MainHand);
          }

          if let Some(crystal_slot) = bot.inventory_find_item(ItemKind::EndCrystal) {
            bot.inventory_move_item(&index, ItemKind::EndCrystal, crystal_slot, 36, false).await;
            bot.start_use_item_by(InteractionHand::MainHand);
          }

          bot.swing_arm();
        });

        sleep!(45);
      } else {
        sleep!(150);
      }
    }
  }
}

impl FastrixiModule<AutoCrystalOptions> for AutoCrystalModule {
  fn new() -> Self {
    Self
  }

  async fn switch(&self, index: u8, options: Arc<AutoCrystalOptions>) -> bool {
    if options.state == 1 && !gettskact(&index, TaskName::Killaura).await {
      let dist = options.distance.unwrap_or(5.5);
      let self_dmg = options.max_self_damage.unwrap_or(10.0);
      let task_handle = tokio::spawn(async move {
        Self::execute_crystal_loop(index, dist, self_dmg).await;
      });
      pushrtsk(&index, TaskName::Killaura, task_handle).await;
      true
    } else if options.state == 0 && gettskact(&index, TaskName::Killaura).await {
      killtsk(&index, TaskName::Killaura).await;
      true
    } else {
      false
    }
  }
}
