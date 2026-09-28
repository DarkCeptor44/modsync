import { translations, type TranslationKey } from './translations';

export type Language = keyof typeof translations;

function detectInitialLanguage(): Language {
	if (typeof window === 'undefined') return 'en';

	const saved = localStorage.getItem('app_lang') as Language | null;
	if (saved && saved in translations) return saved;

	const browserLang = navigator.language;
	if (browserLang.startsWith('pt')) {
		return 'pt-BR';
	}

	return 'en';
}

class I18nState {
	currentLanguage = $state<Language>('en');

	constructor() {
		if (typeof window !== 'undefined') {
			this.currentLanguage = detectInitialLanguage();
		}
	}

	setLanguage(lang: Language) {
		this.currentLanguage = lang;
		if (typeof window !== 'undefined') {
			localStorage.setItem('app_lang', lang);
		}
	}

	get availableLanguages(): Array<{ code: Language; label: string }> {
		return (Object.keys(translations) as Language[]).map((code) => ({
			code,
			label: translations[code].meta.label
		}));
	}

	t(keyPath: TranslationKey, params?: Record<string, string>): string {
		const keys = keyPath.split('.');
		let current: any = translations[this.currentLanguage];

		for (const k of keys) {
			if (current && typeof current === 'object' && k in current) {
				current = current[k];
			} else {
				current = this.getFallback(keyPath);
				break;
			}
		}

		if (typeof current !== 'string') return keyPath;

		if (params) {
			return Object.entries(params).reduce(
				(acc, [pKey, pVal]) => acc.replaceAll(`{${pKey}}`, pVal),
				current
			);
		}

		return current;
	}

	private getFallback(keyPath: string): string {
		const keys = keyPath.split('.');
		let current: any = translations.en;

		for (const k of keys) {
			if (current && typeof current === 'object' && k in current) {
				current = current[k];
			} else {
				return keyPath;
			}
		}

		return typeof current === 'string' ? current : keyPath;
	}
}

export const i18n = new I18nState();
export const t = (key: TranslationKey, params?: Record<string, string>) => i18n.t(key, params);
