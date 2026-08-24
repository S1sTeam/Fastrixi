use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use azalea::BlockPos;
use bytes::Bytes;
use fastrixi_extensions::buffer::BufferExt;
use std::sync::Arc;

use crate::bot::extensions::{BotDefaultExt, BotInteractExt, BotInventoryExt, BotRotationExt, EntityFilter};
use crate::bot::systems::states::{getst, State};
use crate::bot::systems::tasks::{gettskact, killtsk, pushrtsk, TaskName};
use crate::bot::traits::FastrixiModule;
use crate::{sleep, take_bot};

pub struct AutoAnchorOptions {
  pub distance: Option<f64>,
  pub state: u8,
}

impl AutoAnchorOptions {
  pub fn from_bytes(buf: &mut Bytes) -> Option<Self> {
    Some(Self {
      distance: Option::read(buf)?,
      state: u8::read(buf)?,
    })
  }
}

pub struct AutoAnchorModule;

impl AutoAnchorModule {
  pub fn new() -> Self {
    Self
  }

  async fn execute_anchor_loop(index: u8, target_distance: f64) {
    loop {
      if !getst(&index, State::CanAttacking).await {
        sleep!(100);
        continue;
      }

      let mut target_pos = None;

      take_bot!(&index, async |bot| {
        if let Some(target) = bot.find_nearest_entity(&EntityFilter::Player, target_distance) {
          target_pos = Some(bot.get_entity_position(target));
        }
      });

      if let Some(pos) = target_pos {
        let anchor_pos = BlockPos::new(pos.x.floor() as i32, pos.y.floor() as i32, pos.z.floor() as i32);

        take_bot!(&index, async |bot| {
          bot.look_at_block(anchor_pos, false).await;

          if let Some(slot) = bot.inventory_find_item(ItemKind::RespawnAnchor) {
            bot.inventory_move_item(&index, ItemKind::RespawnAnchor, slot, 36, false).await;
            bot.start_use_item_by(InteractionHand::MainHand);
            sleep!(25);
          }

          if let Some(glow_slot) = bot.inventory_find_item(ItemKind::Glowstone) {
            bot.inventory_move_item(&index, ItemKind::Glowstone, glow_slot, 36, false).await;
            bot.start_use_item_by(InteractionHand::MainHand);
            sleep!(25);
          }

          bot.swing_arm();
          bot.start_use_item_by(InteractionHand::MainHand);
        });

        sleep!(60);
      } else {
        sleep!(200);
      }
    }
  }
}

impl FastrixiModule<AutoAnchorOptions> for AutoAnchorModule {
  fn new() -> Self {
    Self
  }

  async fn switch(&self, index: u8, options: Arc<AutoAnchorOptions>) -> bool {
    if options.state == 1 && !gettskact(&index, TaskName::Killaura).await {
      let dist = options.distance.unwrap_or(5.0);
      let task_handle = tokio::spawn(async move {
        Self::execute_anchor_loop(index, dist).await;
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
