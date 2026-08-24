use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use azalea::BlockPos;
use bytes::Bytes;
use fastrixi_extensions::buffer::BufferExt;
use std::sync::Arc;

use crate::bot::extensions::{BotDefaultExt, BotInteractExt, BotInventoryExt, BotRotationExt};
use crate::bot::systems::tasks::{gettskact, killtsk, pushrtsk, TaskName};
use crate::bot::traits::FastrixiModule;
use crate::{sleep, take_bot};

pub struct SmartSurroundOptions {
  pub auto_web: u8,
  pub state: u8,
}

impl SmartSurroundOptions {
  pub fn from_bytes(buf: &mut Bytes) -> Option<Self> {
    Some(Self {
      auto_web: u8::read(buf)?,
      state: u8::read(buf)?,
    })
  }
}

pub struct SmartSurroundModule;

impl SmartSurroundModule {
  pub fn new() -> Self {
    Self
  }

  async fn execute_surround_loop(index: u8, auto_web: bool) {
    loop {
      let mut feet_pos = None;

      take_bot!(&index, async |bot| {
        feet_pos = bot.foot_pos();
      });

      if let Some(pos) = feet_pos {
        let base = BlockPos::new(pos.x.floor() as i32, pos.y.floor() as i32, pos.z.floor() as i32);

        let offsets = [
          BlockPos::new(base.x + 1, base.y, base.z),
          BlockPos::new(base.x - 1, base.y, base.z),
          BlockPos::new(base.x, base.y, base.z + 1),
          BlockPos::new(base.x, base.y, base.z - 1),
        ];

        take_bot!(&index, async |bot| {
          if let Some(obs_slot) = bot.inventory_find_item(ItemKind::Obsidian) {
            bot.inventory_move_item(&index, ItemKind::Obsidian, obs_slot, 36, false).await;

            for target in offsets {
              bot.look_at_block(target, false).await;
              bot.start_use_item_by(InteractionHand::MainHand);
            }
          }

          if auto_web {
            if let Some(web_slot) = bot.inventory_find_item(ItemKind::Cobweb) {
              bot.inventory_move_item(&index, ItemKind::Cobweb, web_slot, 36, false).await;
              bot.look_at_block(base, false).await;
              bot.start_use_item_by(InteractionHand::MainHand);
            }
          }
        });

        sleep!(150);
      } else {
        sleep!(200);
      }
    }
  }
}

impl FastrixiModule<SmartSurroundOptions> for SmartSurroundModule {
  fn new() -> Self {
    Self
  }

  async fn switch(&self, index: u8, options: Arc<SmartSurroundOptions>) -> bool {
    if options.state == 1 && !gettskact(&index, TaskName::Scaffold).await {
      let web = options.auto_web == 1;
      let task_handle = tokio::spawn(async move {
        Self::execute_surround_loop(index, web).await;
      });
      pushrtsk(&index, TaskName::Scaffold, task_handle).await;
      true
    } else if options.state == 0 && gettskact(&index, TaskName::Scaffold).await {
      killtsk(&index, TaskName::Scaffold).await;
      true
    } else {
      false
    }
  }
}
