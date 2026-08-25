import { invoke } from "@tauri-apps/api/core";
import { logger } from "../utils/logger";

/** Модуль управления прокси и прокси-сборщика */
class ProxyModule {
	private proxyList: HTMLTextAreaElement | null;
	private counter: HTMLElement | null;
	private status: HTMLElement | null;
	private rawProxiesCache: string[];
	private currentFilter: "all" | "socks5" | "socks4" | "http";

	constructor() {
		this.proxyList = null;
		this.counter = null;
		this.status = null;
		this.rawProxiesCache = [];
		this.currentFilter = "all";
	}

	/** Метод инициализации функций, связанных со сборщиком и фильтрацией прокси */
	public async init(): Promise<void> {
		this.proxyList = document.getElementById("proxy-list") as HTMLTextAreaElement;
		this.counter = document.getElementById("proxy-counter") as HTMLElement;
		this.status = document.getElementById("proxy-finder-status") as HTMLElement;

		this.proxyList.addEventListener("input", () => {
			this.cacheCurrentInput();
			this.updateCounter();
		});

		document.getElementById("clear-proxy-list")?.addEventListener("click", () => {
			this.proxyList!.value = "";
			this.rawProxiesCache = [];
			this.updateCounter("0");
		});

		document.getElementById("find-proxies")?.addEventListener("click", () => this.collectProxies());
		document.getElementById("check-proxies")?.addEventListener("click", () => this.checkProxies());

		document.getElementById("filter-all-proxies")?.addEventListener("click", () => this.applyFilter("all"));
		document.getElementById("filter-socks5-proxies")?.addEventListener("click", () => this.applyFilter("socks5"));
		document.getElementById("filter-socks4-proxies")?.addEventListener("click", () => this.applyFilter("socks4"));
		document.getElementById("filter-http-proxies")?.addEventListener("click", () => this.applyFilter("http"));
		document.getElementById("filter-remove-duplicates")?.addEventListener("click", () => this.removeDuplicates());

		this.cacheCurrentInput();
		this.updateCounter();
	}

	/** Сохранение текущего содержимого в кэш */
	private cacheCurrentInput(): void {
		if (!this.proxyList) return;
		const lines = this.proxyList.value
			.split("\n")
			.map(l => l.trim())
			.filter(l => l.length > 0);
		if (this.currentFilter === "all" || this.rawProxiesCache.length === 0) {
			this.rawProxiesCache = lines;
		}
	}

	/** Применение протокольного фильтра к списку прокси */
	public applyFilter(filter: "all" | "socks5" | "socks4" | "http"): void {
		this.currentFilter = filter;
		this.cacheCurrentInput();

		// Обновляем визуальный активный стиль кнопок
		const filterButtons = ["all", "socks5", "socks4", "http"];
		filterButtons.forEach(f => {
			const btn = document.getElementById(`filter-${f}-proxies`);
			if (btn) {
				if (f === filter) {
					btn.style.background = "#27272a";
					btn.style.color = "#fff";
					btn.style.borderColor = "#3f3f46";
				} else {
					btn.style.background = "#141418";
					btn.style.color = "#a1a1aa";
					btn.style.borderColor = "#27272a";
				}
			}
		});

		if (filter === "all") {
			this.proxyList!.value = this.rawProxiesCache.join("\n");
		} else {
			const filtered = this.rawProxiesCache.filter(p => {
				const lower = p.toLowerCase();
				if (filter === "socks5") return lower.startsWith("socks5://") || (!lower.includes("://") && lower.includes(":1080"));
				if (filter === "socks4") return lower.startsWith("socks4://");
				if (filter === "http") return lower.startsWith("http://") || lower.startsWith("https://") || (!lower.includes("://") && (lower.includes(":8080") || lower.includes(":3128")));
				return true;
			});
			this.proxyList!.value = filtered.join("\n");
		}

		this.updateCounter();
	}

	/** Удаление повторяющихся строк прокси */
	public removeDuplicates(): void {
		if (!this.proxyList) return;
		const lines = this.proxyList.value
			.split("\n")
			.map(l => l.trim())
			.filter(l => l.length > 0);

		const uniqueSet = Array.from(new Set(lines));
		this.proxyList.value = uniqueSet.join("\n");
		this.rawProxiesCache = uniqueSet;
		this.updateCounter(uniqueSet.length.toString());
		logger.log(`Удалены дубликаты прокси. Уникальных: ${uniqueSet.length}`, "system");
	}

	/** Метод обновления счётчика прокси */
	private updateCounter(value?: string): void {
		if (value) {
			this.counter!.innerText = value;
			return;
		}

		let text = this.proxyList!.value;

		if (!text) {
			this.counter!.innerText = "0";
			return;
		}

		let regex = /(?:\w+:\/\/)?(?:(?:\d{1,3}\.){3}\d{1,3}:\d+)/g;
		let matches = text.match(regex);
		this.counter!.innerText = matches ? matches.length.toString() : "0";
	}

	/** Вспомогательный метод установки статуса поиска прокси */
	private setStatus(text: string, color?: string): void {
		if (!this.status) return;
		this.status!.style.color = color ?? "#848080";
		this.status!.innerText = text;
	}

	/** Метод сборки прокси */
	private async collectProxies(): Promise<void> {
		try {
			this.proxyList!.value = "";

			const algorithm = (document.getElementById("proxy-finder_select_algorithm") as HTMLSelectElement).value;
			const country = (document.getElementById("proxy-finder_select_country") as HTMLSelectElement).value;
			const port = (document.getElementById("proxy-finder_select_port") as HTMLSelectElement).value;
			const count = (document.getElementById("proxy-finder_select_count") as HTMLInputElement).value;

			this.setStatus("Поиск прокси...");

			const bytes = await invoke<number[]>("collect_proxies", {
				options: {
					algorithm: algorithm,
					country: country,
					port: port,
					count: count,
				}
			});

			const uint8arr = new Uint8Array(bytes);
			const decoder = new TextDecoder("utf-8");
			const str = decoder.decode(uint8arr);

			if (str === "") {
				this.setStatus("Ошибка поиска", "#cc1d1dff");
				logger.log("Ошибка сборщика прокси: Не удалось найти прокси", "error");
				return;
			}

			const lines = str.split("\n").filter(l => l.trim().length > 0);
			this.rawProxiesCache = lines;
			this.currentFilter = "all";
			this.proxyList!.value = lines.join("\n");
			this.updateCounter(lines.length.toString());
			this.setStatus("Поиск окончен", "#0cd212ff");
			logger.log(`Сборщик прокси: Загружено ${lines.length} прокси`, "system");
		} catch (error) {
			this.setStatus("Ошибка поиска", "#cc1d1dff");
			logger.log(`Ошибка сборщика прокси: ${error}`, "error");
		} finally {
			setTimeout(() => this.setStatus("Поиск неактивен"), 2500);
		}
	}

	/** Метод проверки прокси */
	private async checkProxies(): Promise<void> {
		try {
			const proxies = this.proxyList?.value;
			if (!proxies) return;

			this.setStatus("Проверка пинга...", "#38bdf8");
			this.updateCounter("0");

			const encoder = new TextEncoder();
			const inbytes = encoder.encode(proxies);

			const out = await invoke<number[]>("check_proxies", {
				bytes: Array.from(inbytes),
			});

			const outbytes = new Uint8Array(out);
			const decoder = new TextDecoder("utf-8");
			const str = decoder.decode(outbytes);

			if (str === "") {
				this.setStatus("Нет рабочих", "#cc1d1d");
				return;
			}

			const lines = str.split("\n").filter(l => l.trim().length > 0);
			this.rawProxiesCache = lines;
			this.proxyList!.value = lines.join("\n");
			this.updateCounter(lines.length.toString());
			this.setStatus("Проверка завершена", "#0cd212");
			logger.log(`Проверка прокси: Найдено ${lines.length} живых прокси`, "system");
		} catch (error) {
			logger.log(`Ошибка проверки прокси: ${error}`, "error");
			this.setStatus("Ошибка проверки", "#cc1d1d");
		} finally {
			setTimeout(() => this.setStatus("Поиск неактивен"), 2500);
		}
	}
}

const proxyModule = new ProxyModule();

export { proxyModule }
