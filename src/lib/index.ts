import { PersistedState } from 'runed';
import type { Theme } from './theme';
import { createToaster } from '@skeletonlabs/skeleton-svelte';

export const theme = new PersistedState<Theme>('theme', 'vintage', { storage: 'local' });

export const toaster = createToaster();
