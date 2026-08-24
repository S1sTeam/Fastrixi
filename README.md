<div align="center">

# ⚡ Fastrixi Client (Advanced Next-Gen Fork)

<p align="center">
  <img src="assets/banner.png" alt="Fastrixi Banner" width="520" />
</p>

### Высокопроизводительный клиент и ботнет-экосистема для Minecraft на базе Rust (Azalea) & Tauri 2
*Премиальный тёмный металлический интерфейс (Dark Metallic), обход защит Limbo/NullCordX/Baroness, боевой модуль Combat 3.0, интеллектуальный авто-фарм и координация роя (Swarm Hivemind).*

[![Release](https://img.shields.io/badge/Release-v1.2.0-success?style=for-the-badge)](https://github.com/S1sTeam/Fastrixi/releases)
[![License](https://img.shields.io/badge/License-GPL--3.0-blue?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-Nightly-orange?style=for-the-badge)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blueviolet?style=for-the-badge)](https://tauri.app/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey?style=for-the-badge)](https://github.com/S1sTeam/Fastrixi/releases)

[📦 Скачать релиз](#-таблица-загрузок--downloads-matrix) • [🌟 Возможности и модули](#-таблица-боевых-и-автоматизированных-модулей) • [🌌 Обходы защит](#-матрица-обходов-защит-и-античитов) • [🛠️ Сборка из исходников](#-сборка-из-исходников)

---

</div>

## 📦 Таблица загрузок / Downloads Matrix

| Платформа / OS | Архитектура | Формат файла | Описание сборщика | Прямая ссылка на скачивание |
| :--- | :--- | :--- | :--- | :--- |
| **🪟 Windows** | x64 (64-bit) | `.exe` | Удобный установщик со встроенным автообновлением | [📥 **Скачать .exe**](https://github.com/S1sTeam/Fastrixi/releases/download/v1.2.0/Fastrixi_1.2.0_x64-setup.exe) |
| **🪟 Windows** | x64 (64-bit) | `.msi` | Windows Installer Package (корпоративное развертывание) | [📥 **Скачать .msi**](https://github.com/S1sTeam/Fastrixi/releases/download/v1.2.0/Fastrixi_1.2.0_x64_en-US.msi) |
| **🐧 Linux** | x86_64 / amd64 | `.AppImage` | Универсальный исполняемый файл (Ubuntu, Arch, Fedora) | [📥 **Скачать .AppImage**](https://github.com/S1sTeam/Fastrixi/releases/download/v1.2.0/Fastrixi_1.2.0_amd64.AppImage) |
| **🐧 Linux** | x86_64 / amd64 | `.deb` | Нативный пакет для Debian, Ubuntu, Linux Mint | [📥 **Скачать .deb**](https://github.com/S1sTeam/Fastrixi/releases/download/v1.2.0/Fastrixi_1.2.0_amd64.deb) |
| **🐧 Linux** | x86_64 | `.rpm` | Пакет для RedHat, Fedora, CentOS, openSUSE | [📥 **Скачать .rpm**](https://github.com/S1sTeam/Fastrixi/releases/download/v1.2.0/Fastrixi-1.2.0-1.x86_64.rpm) |
| **🍏 macOS** | Apple Silicon (M1/M2/M3/M4) | `.dmg` | Образ диска для чипов Apple Silicon | [📥 **Скачать .dmg**](https://github.com/S1sTeam/Fastrixi/releases/download/v1.2.0/Fastrixi_1.2.0_aarch64.dmg) |
| **🍏 macOS** | Apple Silicon (M1/M2/M3/M4) | `.app.tar.gz` | Портативный tar-архив приложения | [📥 **Скачать .tar.gz**](https://github.com/S1sTeam/Fastrixi/releases/download/v1.2.0/Fastrixi_aarch64.app.tar.gz) |

---

## 🌟 Ключевые возможности и модули

### 📊 Таблица боевых и автоматизированных модулей

| Модуль | Категория | Скорость / TPS | Описание функционала |
| :--- | :--- | :--- | :--- |
| **💥 Auto-Anchor & Glowstone** | Combat 3.0 | 20 TPS | Мгновенная постановка Якоря Возрождения, зарядка 1 светокамнем и детонация без задержек |
| **💎 Auto-Crystal 2.0** | Combat 3.0 | 20 TPS | Расчёт безопасного урона, скоростная постановка обсидиана, кристалла края и подрыв за 1 тик |
| **🛡️ Smart Surround & Anti-City** | Combat 3.0 | 1 Tick | Защитная коробка из обсидиана вокруг ног + режим авто-паутины (*Auto-Web*) |
| **🎯 Legit Silent Aim** | Combat 3.0 | Плавный | Сплайны Безье с микро-погрешностями человека в обход GrimAC, Matrix и Vulcan |
| **🎣 Auto-Fish 2.0** | Farm 3.0 | Пакетный | Пакетная авто-рыбалка с авто-подсечкой, починкой Mending и выбросом мусора |
| **⛏️ Auto-Miner 3D** | Farm 3.0 | Адаптивный | X-Ray сканирование чанков, поиск ценных руд и 3D-прокапывание с обходом лавы |
| **🪓 Auto-Woodcutter** | Farm 3.0 | Адаптивный | Сруб деревьев любой высоты снизу вверх + автоматическая посадка саженцев |
| **📦 Smart Chest Sorter** | Economy 3.0 | Мгновенный | Сортировка добычи по сундукам и авто-продажа через `/sell` |
| **👥 Swarm Formations & Guard** | Hivemind 3.0 | Синхронный | Построения роя (фаланга, шеренга, круг) и охрана территории от врагов |

---

## 🌌 Матрица обходов защит и античитов

| Защита / Антибот | Механизм проверки | Реализованный алгоритм обхода |
| :--- | :--- | :--- |
| **LimboFilter / LimboAPI** | Гравитационное ускорение падения в пустоте | Эмуляция физики 20 TPS ($v_y = (v_y - 0.08) \times 0.98$) + авто-ответ на `AcceptTeleportation` |
| **BaronessAuth** | Интерактивные чат-компоненты и карты-капчи | JSON-парсер `clickEvent: run_command` + шумоподавление и OCR карт |
| **NullCordX / FlameCord** | Pre-Join проверки и burst rate limiting | Эмуляция `StatusRequest` перед входом + джиттер `KeepAlive` (40–120 мс) |
| **GrimAC / Matrix / Vulcan** | Проверки резких углов обзора и кликов | Легитные кривые Безье, случайные погрешности движения и Raycast валидация |

---

## 🎨 Премиальный Dark Metallic UI
* **Глубокий фон:** Carbon Black (`#09090b`) со стальными панелями (`#141418`) и акцентами (`#27272a`).
* **Элементы управления:** Серебристо-металлический градиент и платиновые кнопки.
* **Оптимизация:** Оптимизация DOM-графиков RAM/CPU через `requestAnimationFrame`.

---

## 🛠️ Сборка из исходников

### Требования
* **Rust:** `nightly` toolchain (`nightly-2026-01-15`)
* **Node.js:** v20+ / v22 LTS
* **npm** или **pnpm**

```bash
# 1. Клонирование репозитория
git clone https://github.com/S1sTeam/Fastrixi.git
cd Fastrixi

# 2. Установка зависимостей
npm install

# 3. Сборка веб-ресурсов
npm run build

# 4. Сборка готового бинарника
npm run tauri build
```

---

<div align="center">
  <sub>Разработано с ❤️ командой S1sTeam • 2026</sub>
</div>
