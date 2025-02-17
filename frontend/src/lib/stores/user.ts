import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

export interface User {
    id: string;
    username: string;
    email: string;
    avatar: string;
    profile_picture?: string;
}

interface UserState {
    user: User | null;
    loading: boolean;
    error: string | null;
}

const initialState: UserState = {
    user: null,
    loading: true,
    error: null
};

function createUserStore() {
    const { subscribe, set, update } = writable<UserState>(initialState);

    return {
        subscribe,
        setUser: (user: User | null) => {
            update(state => ({ ...state, user, loading: false }));
        },
        setError: (error: string) => update(state => ({ ...state, error, loading: false })),
        setLoading: (loading: boolean) => update(state => ({ ...state, loading })),
        reset: () => set(initialState),
        logout: () => {
            set({ user: null, loading: false, error: null });
        }
    };
}

export const userStore = createUserStore();
export const isAuthenticated = derived(userStore, $userStore => !!$userStore.user); 