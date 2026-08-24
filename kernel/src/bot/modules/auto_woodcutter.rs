use azalea::core::direction::Direction;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::protocol::packets::game::s_player_action::Action;
use azalea::protocol::packets::game::ServerboundPlayerAction;
use azalea::registry::builtin::ItemKind;
use azalea::BlockPos;
use bytes::Bytes;
use fastrixi_extensions::buffer::BufferExt;
use std::sync::Arc;

use crate::bot::extensions::{BotDefaultExt, BotInteractExt, BotInventoryExt, BotRotationExt};
use crate::bot::systems::tasks::{gettskact, killtsk, pushrtsk, TaskName};
use crate::bot::traits::FastrixiModule;
use crate::{sleep, take_bot};

pub struct AutoWoodcutterOptions {
  pub radius: Option<i32>,
  pub auto_replant: u8,
  pub state: u8,
}

impl AutoWoodcutterOptions {
  pub fn from_bytes(buf: &mut Bytes) -> Option<Self> {
    Some(Self {
      radius: Option::read(buf)?,
      auto_replant: u8::read(buf)?,
      state: u8::read(buf)?,
    })
  }
}

pub struct AutoWoodcutterModule;

impl AutoWoodcutterModule {
  pub fn new() -> Self {
    Self
  }

  async fn execute_woodcutter_loop(index: u8, search_radius: i32, auto_replant: bool) {
    loop {
      let mut bot_pos = None;

      take_bot!(&index, async |bot| {
        bot_pos = bot.foot_pos();
      });

      if let Some(pos) = bot_pos {
        let base_x = pos.x.floor() as i32;
        let base_y = pos.y.floor() as i32;
        let base_z = pos.z.floor() as i32;

        take_bot!(&index, async |bot| {
          if let Some(axe_slot) = bot
            .inventory_find_item(ItemKind::DiamondAxe)
            .or_else(|| bot.inventory_find_item(ItemKind::NetheriteAxe))
            .or_else(|| bot.inventory_find_item(ItemKind::IronAxe))
          {
            bot.inventory_move_item(&index, ItemKind::DiamondAxe, axe_slot, 36, false).await;
          }

          for dy in 0..6 {
            for dx in -search_radius..=search_radius {
              for dz in -search_radius..=search_radius {
                let target_block = BlockPos::new(base_x + dx, base_y + dy, base_z + dz);
                bot.look_at_block(target_block, false).await;

                bot.write_packet(ServerboundPlayerAction {
                  action: Action::StartDestroyBlock,
                  pos: target_block,
                  direction: Direction::Up,
                  seq: 0,
                });

                bot.swing_arm();
                sleep!(200);

                bot.write_packet(ServerboundPlayerAction {
                  action: Action::StopDestroyBlock,
                  pos: target_block,
                  direction: Direction::Up,
                  seq: 0,
                });
              }
            }
          }

          if auto_replant {
            if let Some(sapling_slot) = bot
              .inventory_find_item(ItemKind::OakSapling)
              .or_else(|| bot.inventory_find_item(ItemKind::BirchSapling))
              .or_else(|| bot.inventory_find_item(ItemKind::SpruceSapling))
            {
              bot.inventory_move_item(&index, ItemKind::OakSapling, sapling_slot, 36, false).await;
              let ground_pos = BlockPos::new(base_x, base_y, base_z);
              bot.look_at_block(ground_pos, false).await;
              bot.start_use_item_by(InteractionHand::MainHand);
            }
          }
        });

        sleep!(1000);
      } else {
        sleep!(500);
      }
    }
  }
}

impl FastrixiModule<AutoWoodcutterOptions> for AutoWoodcutterModule {
  fn new() -> Self {
    Self
  }

  async fn switch(&self, index: u8, options: Arc<AutoWoodcutterOptions>) -> bool {
    if options.state == 1 && !gettskact(&index, TaskName::Miner).await {
      let r = options.radius.unwrap_or(4);
      let replant = options.auto_replant == 1;
      let task_handle = tokio::spawn(async move {
        Self::execute_woodcutter_loop(index, r, replant).await;
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
