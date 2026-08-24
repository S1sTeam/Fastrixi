<div align="center">

# ⚡ Fastrixi Client (Advanced Next-Gen Fork)

<p align="center">
  <img src="interface/assets/logo.png" alt="Fastrixi Logo" width="160" />
</p>

**Высокопроизводительный клиент и ботнет-экосистема для Minecraft на базе Rust (Azalea) и Tauri 2.**
*Премиальный тёмный металлический интерфейс (Dark Metallic), обход защит Limbo/NullCordX/Baroness, боевой модуль Combat 3.0, интеллектуальный авто-фарм и координация роя (Swarm Hivemind).*

[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/tauri-v2-blueviolet.svg)](https://tauri.app/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com/S1sTeam/Fastrixi/releases)

[🇷🇺 Документация на русском](#-особенности-и-модули) • [🇬🇧 English Documentation](#-key-features) • [📦 Скачать релиз](https://github.com/S1sTeam/Fastrixi/releases)

---

</div>

## 🌟 Особенности и модули

### 🎨 1. Dark Metallic & Glassmorphism UI 2.0
- **Эстетика тёмного монохромного металла:** Глубокий карбоновый фон (`#09090b`), серебристо-металлический градиент, графитовые панели и стальные акценты.
- **Интерактивные графики:** Оптимизированный мониторинг нагрузки CPU и памяти RAM в реальном времени.
- **Компактный режим (Mini-Widget):** Сворачивание в компактный оверлей для удобного контроля.

### 🌌 2. Движок обхода продвинутых защит
- **LimboAPI & LimboFilter Bypass:** Точная симуляция 20 TPS гравитации и физики падения в пустоте ($v_y = (v_y - 0.08) \times 0.98$), мгновенная синхронизация `AcceptTeleportation` и spoofing брендов клиента.
- **BaronessAuth & Captcha Solver:** Авто-кликер по интерактивным JSON-компонентам чата (`clickEvent: run_command` / `suggest_command`), фильтрация шума и OCR карт-капчи.
- **NullCordX / FlameCord Rate-Limiter:** Эмуляция Pre-Join пинга (`StatusRequest`) и джиттер KeepAlive пакетов (40–120 мс).

### ⚔️ 3. Боевой комплекс (Combat & PvP 3.0)
- 💥 **Auto-Anchor & Glowstone:** Мгновенная постановка Якоря Возрождения, зарядка светокамнем и детонация без задержек.
- 💎 **Auto-Crystal 2.0:** Калькулятор безопасного урона, скоростная постановка обсидиана, кристалла края и подрыв за 1 тик.
- 🛡️ **Smart Surround & Anti-City:** Быстрое окружение ног обсидианом + режим авто-паутины (*Auto-Web*).
- 🎯 **Legit Silent Aim:** Плавное наведение по сплайнам Безье с микро-погрешностями человека в обход античитов GrimAC, Matrix, Vulcan.

### 🌾 4. Авто-Гринд и Экономика (Auto-Farm 3.0)
- 🎣 **Auto-Fish 2.0:** Пакетная авто-рыбалка с авто-подсечкой, авто-починкой Mending и выбросом мусора.
- ⛏️ **Auto-Miner 3D:** X-Ray сканирование чанков, поиск ценных руд и безопасный 3D-путь с обходом лавы.
- 🪓 **Auto-Woodcutter:** Сруб деревьев снизу вверх + автоматическая посадка саженцев.
- 📦 **Smart Chest Sorter & Auto-Sell:** Сортировка добычи по категориям сундуков и авто-продажа через `/sell`.

### 👥 5. Управление Роем (Swarm & Hivemind 3.0)
- **Построения роя:** Шеренга, колонна, защитный круг, фаланга со щитами.
- **Guard Mode:** Автоматическое патрулирование территории и атака вражеских игроков.
- **Auto-Reconnect & Proxy Rotator:** Автоматическая смена прокси на рабочий при получении IP-бана.

### 📡 6. Анализатор серверов и Прокси-Хаб
- **Deep Server Ping & SRV Resolver:** Определение реального IP/порта за доменом, детект ядра сервера (Velocity, Bungee, Paper, Purpur, NullCordX).
- **Ростер игроков:** Таблица игроков с аватарками скинов и экспорт в TXT/JSON.
- **Advanced Proxy Manager:** Интерактивная таблица с живым пингом, фильтрами SOCKS4/5/HTTP и режимами ротации.

---

## 🛠️ Сборка из исходников

### Требования
* **Rust:** `nightly` toolchain
* **Node.js:** v18+ (рекомендуется v20 LTS)
* **npm** или **pnpm**

### Команды сборки

```bash
# 1. Клонирование репозитория
git clone https://github.com/S1sTeam/Fastrixi.git
cd Fastrixi

# 2. Установка зависимостей интерфейса
npm install

# 3. Сборка веб-ресурсов
npm run build

# 4. Запуск в режиме разработки
npm run tauri dev

# 5. Сборка готового дистрибутива
npm run tauri build
```

---

## 📜 Лицензия

Проект распространяется под свободной лицензией **GPL-3.0**. Подробности в файле [LICENSE](LICENSE).

<div align="center">
  <sub>Разработано с ❤️ командой S1sTeam</sub>
</div>
