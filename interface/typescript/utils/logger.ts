import { date } from "./date";

/** Структура статистики логгера */
interface LoggerStatistics {
	index: number;
	visible: {
		system: boolean;
		extended: boolean;
	};
}

class Logger {
	private journal: HTMLDivElement | null;
	private statistics: LoggerStatistics;

	constructor() {
		this.journal = null;
		this.statistics = {
			index: 0,
			visible: {
				system: true,
				extended: false
			}
		};
	}

	/** Метод инициализации логгера и связанных с ним функций */
	public init(): void {
		this.journal = document.getElementById("log-content") as HTMLDivElement;
		document.getElementById("clear-journal")?.addEventListener("click", () => this.clear());
		document.getElementById("export-journal-btn")?.addEventListener("click", () => this.exportLogs());
		document.getElementById("journal-search-input")?.addEventListener("input", (e) => {
			const query = (e.target as HTMLInputElement).value;
			this.filterLogs(query);
		});
	}

	/** Экспорт журнала в файл .txt */
	public exportLogs(): void {
		if (!this.journal) return;
		const lines = document.querySelectorAll(".log-line");
		if (lines.length === 0) {
			this.log("Журнал пуст, нечего экспортировать", "system");
			return;
		}

		let textOutput = `=== Fastrixi Client Logs [${date()}] ===\n\n`;
		lines.forEach(line => {
			const dateText = line.querySelector(".log-line-date")?.textContent || "";
			const contentText = line.querySelector(".log-line-content")?.textContent || "";
			textOutput += `[${dateText}] ${contentText}\n`;
		});

		const blob = new Blob([textOutput], { type: "text/plain;charset=utf-8" });
		const url = URL.createObjectURL(blob);
		const anchor = document.createElement("a");
		const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
		anchor.href = url;
		anchor.download = `fastrixi_logs_${timestamp}.txt`;
		document.body.appendChild(anchor);
		anchor.click();
		document.body.removeChild(anchor);
		URL.revokeObjectURL(url);
		this.log("Логи успешно экспортированы в файл", "system");
	}

	/** Фильтрация строк журнала по поисковому запросу */
	public filterLogs(query: string): void {
		const lowerQuery = query.toLowerCase().trim();
		document.querySelectorAll<HTMLElement>(".log-line").forEach(line => {
			const content = line.querySelector(".log-line-content")?.textContent?.toLowerCase() || "";
			const date = line.querySelector(".log-line-date")?.textContent?.toLowerCase() || "";
			if (lowerQuery === "" || content.includes(lowerQuery) || date.includes(lowerQuery)) {
				const logType = line.getAttribute("log-type");
				if ((logType === "system" && !this.statistics.visible.system) || (logType === "extended" && !this.statistics.visible.extended)) {
					line.style.display = "none";
				} else {
					line.style.display = "flex";
				}
			} else {
				line.style.display = "none";
			}
		});
	}

	/** 
	 * Метод отправки сообщения в журнал.
	 * 
	 * Данный метод проверяет количество существующих сообщений в журнале.
	 * Если количество равняется или превышает 400, то удаляется самое старое сообщение из журнала.
	 * 
	 * Ещё этот метод учитывает текущие состояния видимости определённых типов сообщений.
	 *
	 * Так же при включенной опции "Авто скролл" этот метод будет каждое новое сообщение прокручивать контент до самого низа.
	 */
	public log(text: string, type: string): void {
		if (!this.journal) return;

		if (this.statistics.index >= 400) {
			this.journal.firstChild?.remove();
			this.statistics.index = 399;
		}

		this.statistics.index++;

		const line = document.createElement("div");
		line.className = "log-line";
		if (type === "system" || type === "extended") line.setAttribute("log-type", type);

		line.innerHTML = `
      <div class="log-line-date">${date()}</div>
    `;

		const content = document.createElement("div");
		content.className = `log-line-content ${type}`;
		content.innerText = text;

		if (type === "system") content.style.fontStyle = "italic";
		if ((line.getAttribute("log-type") === "system" && !this.statistics.visible.system) || (line.getAttribute("log-type") === "extended" && !this.statistics.visible.extended)) line.style.display = "none";

		line.appendChild(content);
		this.journal.appendChild(line);

		if ((document.getElementById("journal_chbx_auto-scroll") as HTMLInputElement).checked) this.journal.scrollTo({ top: this.journal.scrollHeight, behavior: "smooth" });
	}

	/** Метод смены видимости определённых типов сообщений в журнале */
	public setVisibility(type: "system" | "extended", state: boolean): void {
		this.statistics.visible[type] = state;
		document.querySelectorAll<HTMLElement>(`[log-type="${type}"]`).forEach(e => e.style.display = state ? "flex" : "none");
	}

	/** Метод полной очистки журнала */
	private clear(): void {
		document.querySelectorAll(".log-line").forEach(e => e.remove());
		this.statistics.index = 0;
	}
}

const logger = new Logger();

export { logger }
