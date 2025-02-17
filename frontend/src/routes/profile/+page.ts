import { browser } from '$app/environment';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { userStore } from '$lib/stores/user';
import { get } from 'svelte/store';

export const load: PageLoad = async () => {
    // Only run on browser
    if (!browser) {
        return {};
    }

    const user = get(userStore).user;
    if (!user) {
        throw redirect(302, '/login');
    }

    return {};
}; 