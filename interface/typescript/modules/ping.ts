import { invoke } from "@tauri-apps/api/core";

import { logger } from "../utils/logger";
import { generateId } from "../utils/generator";

/** Структура информации сервера */
interface ServerInformation {
	ip_address: string;
	port: number;
	latency_ms: number;
	server_icon: string | null;
	protocol_version: number;
	server_version: string;
	server_core: string;
	protection: string;
	description: string;
	players_online: number;
	players_max: number;
	list_of_players: Array<{ username: string; uuid: string; }>;
	success: boolean;
}

/** Модуль управления пинговкой серверов */
class PingModule {
	/** Метод инициализации функций, связанных с пингованием */
	public init(): void {
		document.getElementById("ping-server")?.addEventListener("click", async () => await this.ping_server());
		(document.getElementById("ping-server-address") as HTMLInputElement)?.addEventListener("keydown", async (e) => {
			if (e.key === "Enter") await this.ping_server();
		});
	}

	/** Метод пингования сервера */
	private async ping_server(): Promise<void> {
		try {
			const address = (document.getElementById("ping-server-address") as HTMLInputElement).value.trim();

			if (address === "") return;

			const pingBtn = document.getElementById("ping-server") as HTMLButtonElement;
			if (pingBtn) pingBtn.disabled = true;

			const realIpEl = document.getElementById("ping-real-ip");
			const protectionEl = document.getElementById("ping-protection-type");
			const coreEl = document.getElementById("ping-server-core");
			const latencyEl = document.getElementById("ping-latency-status");

			if (realIpEl) realIpEl.innerText = "Резолв адреса...";
			if (protectionEl) protectionEl.innerText = "Анализ защит...";
			if (coreEl) coreEl.innerText = "Опрос сервера...";
			if (latencyEl) latencyEl.innerText = "Измерение...";

			const result = await invoke<ServerInformation>("get_server_info", {
				address: address
			});

			if (pingBtn) pingBtn.disabled = false;

			if (result.success) {
				if (realIpEl) realIpEl.innerText = `${result.ip_address}:${result.port}`;
				if (protectionEl) protectionEl.innerText = result.protection;
				if (coreEl) coreEl.innerText = result.server_core;
				if (latencyEl) latencyEl.innerText = `${result.latency_ms} ms (${result.players_online}/${result.players_max} игроков)`;
			} else {
				if (realIpEl) realIpEl.innerText = `${result.ip_address}:${result.port}`;
				if (protectionEl) protectionEl.innerText = result.protection;
				if (coreEl) coreEl.innerText = result.server_core;
				if (latencyEl) latencyEl.innerText = "Офлайн / Таймаут";
			}

			const pingInfo = document.getElementById("ping-info") as HTMLElement;
			pingInfo.innerHTML = "";

			if (!result.success) {
				const errorCard = document.createElement("div");
				errorCard.className = "header";
				errorCard.style.color = "#ef4444";
				errorCard.innerText = `Не удалось подключиться к ${address}. Проверьте адрес или порт.`;
				pingInfo.appendChild(errorCard);
				return;
			}

			const card = document.createElement("div");
			card.className = "card";

			const removeBtnId = `remove-ping-card-${generateId()}`;

			card.innerHTML = `
        ${result.server_icon ? `<img class="icon" src="${result.server_icon}" draggable="false">` : ''}
        <div class="text">
          <label>${result.description || result.server_version}</label>
          <div>
            <p>Игроки: ${result.players_online} / ${result.players_max}</p>
            <p>Версия: ${result.server_version}</p>
            <p>IP-адрес: ${result.ip_address}:${result.port}</p>
            <p>Протокол: ${result.protocol_version}</p>
            <p>Задержка (Ping): ${result.latency_ms} ms</p>
          </div>
        </div>

        <button class="min" id="${removeBtnId}">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path stroke="none" d="M0 0h24v24H0z" fill="none" />
            <path d="M18 6l-12 12" />
            <path d="M6 6l12 12" />
          </svg>
        </button>
      `;

			pingInfo.appendChild(card);

			const listOfPlayers = document.createElement("div");
			listOfPlayers.className = "list";
			listOfPlayers.style.display = "none";

			if (result.list_of_players && result.list_of_players.length > 0) {
				listOfPlayers.style.display = "flex";

				const element = document.createElement("div");
				element.className = "element";

				element.innerHTML = `
          <p class="username">Никнейм</p>
          <div class="splitter"></div>
          <p class="uuid">UUID</p>
        `;

				listOfPlayers.appendChild(element);

				for (const player of result.list_of_players) {
					const el = document.createElement("div");
					el.className = "element";

					el.innerHTML = `
            <p class="username">${player.username}</p>
            <div class="splitter"></div>
            <p class="uuid">${player.uuid}</p>
          `;

					listOfPlayers.appendChild(el);
				}

				pingInfo.appendChild(listOfPlayers);
			} else {
				const header = document.createElement("div");
				header.className = "header";
				header.innerText = "Игроки в выборке отсутствуют";
				pingInfo.appendChild(header);
			}

			document.getElementById(removeBtnId)?.addEventListener("click", () => {
				card.remove();
				listOfPlayers.remove();
				pingInfo.innerHTML = "";
			});
		} catch (error) {
			logger.log(`Ошибка пингования сервера: ${error}`, "error");
		}
	}
}

const pingModule = new PingModule();

export { pingModule }
