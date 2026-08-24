use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use bytes::Bytes;
use fastrixi_extensions::buffer::BufferExt;
use std::sync::Arc;

use crate::bot::extensions::{BotDefaultExt, BotInteractExt, BotInventoryExt};
use crate::bot::systems::tasks::{gettskact, killtsk, pushrtsk, TaskName};
use crate::bot::traits::FastrixiModule;
use crate::{sleep, take_bot};

pub struct AutoFishOptions {
  pub auto_repair: u8,
  pub state: u8,
}

impl AutoFishOptions {
  pub fn from_bytes(buf: &mut Bytes) -> Option<Self> {
    Some(Self {
      auto_repair: u8::read(buf)?,
      state: u8::read(buf)?,
    })
  }
}

pub struct AutoFishModule;

impl AutoFishModule {
  pub fn new() -> Self {
    Self
  }

  async fn execute_fish_loop(index: u8, _auto_repair: bool) {
    loop {
      take_bot!(&index, async |bot| {
        if let Some(rod_slot) = bot.inventory_find_item(ItemKind::FishingRod) {
          bot.inventory_move_item(&index, ItemKind::FishingRod, rod_slot, 36, false).await;
          bot.start_use_item_by(InteractionHand::MainHand);
          sleep!(300);
          sleep!(3500);
          bot.start_use_item_by(InteractionHand::MainHand);
          bot.swing_arm();
        }
      });

      sleep!(1000);
    }
  }
}

impl FastrixiModule<AutoFishOptions> for AutoFishModule {
  fn new() -> Self {
    Self
  }

  async fn switch(&self, index: u8, options: Arc<AutoFishOptions>) -> bool {
    if options.state == 1 && !gettskact(&index, TaskName::Miner).await {
      let repair = options.auto_repair == 1;
      let task_handle = tokio::spawn(async move {
        Self::execute_fish_loop(index, repair).await;
      });
      pushrtsk(&index, TaskName::Miner, task_handle).await;
      true
    } else if options.state == 0 && gettskact(&index, TaskName::Miner).await {
      killtsk(&index, TaskName::Miner).await;
      true
    } else {
      false
    }
  }
}
