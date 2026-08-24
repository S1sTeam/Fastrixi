use bytes::Bytes;
use fastrixi_extensions::buffer::BufferExt;
use std::sync::Arc;

use crate::bot::extensions::{BotInventoryExt, ClickMode};
use crate::bot::systems::tasks::{gettskact, killtsk, pushrtsk, TaskName};
use crate::bot::traits::FastrixiModule;
use crate::{sleep, take_bot};

pub struct ChestSorterOptions {
  pub auto_sell_cmd: Option<String>,
  pub state: u8,
}

impl ChestSorterOptions {
  pub fn from_bytes(buf: &mut Bytes) -> Option<Self> {
    Some(Self {
      auto_sell_cmd: Option::read(buf)?,
      state: u8::read(buf)?,
    })
  }
}

pub struct ChestSorterModule;

impl ChestSorterModule {
  pub fn new() -> Self {
    Self
  }

  async fn execute_sorter_loop(index: u8, auto_sell_cmd: Option<String>) {
    loop {
      take_bot!(&index, async |bot| {
        if let Some(menu) = bot.get_inventory_menu() {
          for (slot, item) in menu.slots().iter().enumerate() {
            if !item.is_empty() && slot >= 9 && slot <= 44 {
              bot.inventory_click(&index, slot, ClickMode::Shift, false).await;
              sleep!(30);
            }
          }
        }

        if let Some(cmd) = &auto_sell_cmd {
          if !cmd.is_empty() {
            bot.chat(cmd);
          }
        }
      });

      sleep!(3000);
    }
  }
}

impl FastrixiModule<ChestSorterOptions> for ChestSorterModule {
  fn new() -> Self {
    Self
  }

  async fn switch(&self, index: u8, options: Arc<ChestSorterOptions>) -> bool {
    if options.state == 1 && !gettskact(&index, TaskName::Stealer).await {
      let cmd = options.auto_sell_cmd.clone();
      let task_handle = tokio::spawn(async move {
        Self::execute_sorter_loop(index, cmd).await;
      });
      pushrtsk(&index, TaskName::Stealer, task_handle).await;
      true
    } else if options.state == 0 && gettskact(&index, TaskName::Stealer).await {
      killtsk(&index, TaskName::Stealer).await;
      true
    } else {
      false
    }
  }
}
