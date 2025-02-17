import { browser } from '$app/environment';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { userStore, isAuthenticated } from '$lib/stores/user';
import { get } from 'svelte/store';

export const load: PageLoad = async () => {
    // Only run on browser
    if (!browser) {
        return {};
    }

    const authenticated = get(isAuthenticated);
    if (!authenticated) {
        throw redirect(302, '/login');
    }

    return {};
}; 