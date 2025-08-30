import { theme } from '.';

export const THEMES = [
    'catppuccin',
    'cerberus',
    'concord',
    'crimson',
    'fennec',
    'hamlindigo',
    'legacy',
    'mint',
    'modern',
    'mona',
    'nosh',
    'nouveau',
    'pine',
    'reign',
    'rocket',
    'rose',
    'sahara',
    'seafoam',
    'terminus',
    'vintage',
    'vox',
    'wintry'
] as const;

export type Theme = (typeof THEMES)[number];

export function updateThemeInHtml(newTheme: Theme) {
    const body = document.querySelector('html')!;

    theme.current = newTheme;

    body.dataset.theme = newTheme;
}

export function initTheme() {
    const body = document.querySelector('html')!;

    body.dataset.theme = theme.current;
}
