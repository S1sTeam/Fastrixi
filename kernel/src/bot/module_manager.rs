use std::sync::Arc;

use bytes::Bytes;
use once_cell::sync::Lazy;
use fastrixi_extensions::buffer::BufferExt;
use fastrixi_extensions::index::IndexExt;
use fastrixi_macros::Index;

use crate::bot::modules::*;
use crate::bot::systems::profile::PROFILE_SYSTEM;
use crate::bot::traits::FastrixiModule;
use crate::launch::process::current_options;
use crate::server::transfer::{emit_log, emit_msg};
use crate::webhook::send_webhook;

pub static MODULES: Lazy<ModuleManager> = Lazy::new(|| ModuleManager::new());

#[derive(Debug, Index)]
enum ModuleName {
  Chat = 0,
  Action = 1,
  Inventory = 2,
  Movement = 3,
  AntiAfk = 4,
  Stalker = 5,
  Flight = 6,
  Killaura = 7,
  Scaffold = 8,
  AntiFall = 9,
  BowAim = 10,
  Stealer = 11,
  Miner = 12,
  Farmer = 13,
  AutoAnchor = 14,
  AutoCrystal = 15,
  SmartSurround = 16,
  AutoFish = 17,
  AutoWoodcutter = 18,
  ChestSorter = 19,
  SwarmGuard = 20,
}

impl ModuleName {
  pub fn to_string(&self) -> String {
    match self {
      Self::Chat => "chat",
      Self::Action => "action",
      Self::Inventory => "inventory",
      Self::Movement => "movement",
      Self::AntiAfk => "anti-afk",
      Self::Stalker => "stalker",
      Self::Flight => "flight",
      Self::Killaura => "killaura",
      Self::Scaffold => "scaffold",
      Self::AntiFall => "anti-fall",
      Self::BowAim => "bow-aim",
      Self::Stealer => "stealer",
      Self::Miner => "miner",
      Self::Farmer => "farmer",
      Self::AutoAnchor => "auto-anchor",
      Self::AutoCrystal => "auto-crystal",
      Self::SmartSurround => "smart-surround",
      Self::AutoFish => "auto-fish",
      Self::AutoWoodcutter => "auto-woodcutter",
      Self::ChestSorter => "chest-sorter",
      Self::SwarmGuard => "swarm-guard",
    }
    .to_string()
  }
}

#[derive(Debug)]
struct ControlMessage {
  module_name: ModuleName,
  group: String,
  raw_options: Bytes,
}

impl ControlMessage {
  fn from_bytes(mut buf: Bytes) -> Option<Self> {
    let module_index = u8::read(&mut buf)?;
    let module_name = ModuleName::from_index(module_index)?;
    let group = String::read(&mut buf)?;

    Some(Self {
      module_name,
      group,
      raw_options: buf,
    })
  }
}

pub struct ModuleManager {
  chat: ChatModule,
  action: ActionModule,
  inventory: InventoryModule,
  movement: MovementModule,
  anti_afk: AntiAfkModule,
  stalker: StalkerModule,
  flight: FlightModule,
  killaura: KillauraModule,
  scaffold: ScaffoldModule,
  anti_fall: AntiFallModule,
  bow_aim: BowAimModule,
  stealer: StealerModule,
  miner: MinerModule,
  farmer: FarmerModule,
  auto_anchor: AutoAnchorModule,
  auto_crystal: AutoCrystalModule,
  smart_surround: SmartSurroundModule,
  auto_fish: AutoFishModule,
  auto_woodcutter: AutoWoodcutterModule,
  chest_sorter: ChestSorterModule,
  swarm_guard: SwarmGuardModule,
}

impl ModuleManager {
  pub fn new() -> Self {
    Self {
      chat: ChatModule::new(),
      action: ActionModule::new(),
      inventory: InventoryModule::new(),
      movement: MovementModule::new(),
      anti_afk: AntiAfkModule::new(),
      stalker: StalkerModule::new(),
      flight: FlightModule::new(),
      killaura: KillauraModule::new(),
      scaffold: ScaffoldModule::new(),
      anti_fall: AntiFallModule::new(),
      bow_aim: BowAimModule::new(),
      stealer: StealerModule::new(),
      miner: MinerModule::new(),
      farmer: FarmerModule::new(),
      auto_anchor: AutoAnchorModule::new(),
      auto_crystal: AutoCrystalModule::new(),
      smart_surround: SmartSurroundModule::new(),
      auto_fish: AutoFishModule::new(),
      auto_woodcutter: AutoWoodcutterModule::new(),
      chest_sorter: ChestSorterModule::new(),
      swarm_guard: SwarmGuardModule::new(),
    }
  }

  /// Метод взаимодействия с указанным модулем управления
  pub async fn control(&self, bytes: Bytes) -> Option<()> {
    let mut message = ControlMessage::from_bytes(bytes)?;

    let profiles = PROFILE_SYSTEM.get_all_connected().await;
    let total_count = profiles.len();
    let mut success_count = 0;

    match message.module_name {
      ModuleName::Chat => {
        let options = Arc::new(ChatOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.chat.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::Action => {
        let options = Arc::new(ActionOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.action.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::Inventory => {
        let options = Arc::new(InventoryOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.inventory.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::Movement => {
        let options = Arc::new(MovementOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.movement.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::AntiAfk => {
        let options = Arc::new(AntiAfkOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.anti_afk.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::Stalker => {
        let options = Arc::new(StalkerOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.stalker.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::Flight => {
        let options = Arc::new(FlightOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.flight.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::Killaura => {
        let options = Arc::new(KillauraOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.killaura.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::Scaffold => {
        let options = Arc::new(ScaffoldOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.scaffold.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::AntiFall => {
        let options = Arc::new(AntiFallOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.anti_fall.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::BowAim => {
        let options = Arc::new(BowAimOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.bow_aim.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::Stealer => {
        let options = Arc::new(StealerOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.stealer.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::Miner => {
        let options = Arc::new(MinerOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.miner.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::Farmer => {
        let options = Arc::new(FarmerOptions::from_bytes(&mut message.raw_options)?);

        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }

          if self.farmer.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::AutoAnchor => {
        let options = Arc::new(AutoAnchorOptions::from_bytes(&mut message.raw_options)?);
        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }
          if self.auto_anchor.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::AutoCrystal => {
        let options = Arc::new(AutoCrystalOptions::from_bytes(&mut message.raw_options)?);
        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }
          if self.auto_crystal.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::SmartSurround => {
        let options = Arc::new(SmartSurroundOptions::from_bytes(&mut message.raw_options)?);
        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }
          if self.smart_surround.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::AutoFish => {
        let options = Arc::new(AutoFishOptions::from_bytes(&mut message.raw_options)?);
        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }
          if self.auto_fish.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::AutoWoodcutter => {
        let options = Arc::new(AutoWoodcutterOptions::from_bytes(&mut message.raw_options)?);
        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }
          if self.auto_woodcutter.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::ChestSorter => {
        let options = Arc::new(ChestSorterOptions::from_bytes(&mut message.raw_options)?);
        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }
          if self.chest_sorter.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
      ModuleName::SwarmGuard => {
        let options = Arc::new(SwarmGuardOptions::from_bytes(&mut message.raw_options)?);
        for (index, _) in profiles {
          if let Some(profile) = PROFILE_SYSTEM.get(&index).await {
            if profile.group != message.group {
              continue;
            }
          }
          if self.swarm_guard.switch(index, options.clone()).await {
            success_count += 1;
          }
        }
      }
    }

    let module_name = message.module_name.to_string();

    if let Some(opts) = current_options().await {
      if opts.basic.use_webhook && opts.webhook.send_actions {
        send_webhook(
          opts.webhook.url,
          format!(
            "Группа ботов \"{}\" получила команду \"{}\". Выполнили {} из {} ботов.",
            message.group, module_name, success_count, total_count,
          ),
        );
      }
    }

    emit_log(
      format!(
        "Группа ботов \"{}\" получила команду \"{}\". Выполнили {} из {} ботов",
        message.group, module_name, success_count, total_count,
      ),
      "extended",
    );

    emit_msg(
      "Управление",
      format!(
        "Группа ботов \"{}\" получила команду \"{}\". Выполнили {} из {} ботов.",
        message.group, module_name, success_count, total_count
      ),
    );

    Some(())
  }
}
