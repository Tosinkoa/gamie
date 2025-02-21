import { env } from '$env/dynamic/public';
import { userStore } from '../stores/user';
import type { User } from '../stores/user';

interface LoginCredentials {
    username_or_email: string;
    password: string;
}

interface SignupCredentials {
    username: string;
    email: string;
    password: string;
    password_verify: string;
}

class AuthService {
    private baseUrl: string;
    private isProd: boolean;

    constructor() {
        this.baseUrl = env.PUBLIC_BACKEND_URL;
        this.isProd = env.PUBLIC_ENV === 'production';
    }

    async login(credentials: LoginCredentials): Promise<void> {
        try {
            userStore.setLoading(true);
            console.log('Attempting login with credentials:', { ...credentials, password: '[REDACTED]' });

            const response = await fetch(`${this.baseUrl}/auth/login`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(credentials),
                credentials: 'include',
            });

            console.log('Login response status:', response.status);
            const data = await response.json();
            console.log('Login response data:', data);

            if (!response.ok) {
                throw new Error(data.message || 'Login failed');
            }

            userStore.setUser(data.user);
            console.log('User set in store:', data.user);
        } catch (error) {
            console.error('Login error:', error);
            userStore.setError(error instanceof Error ? error.message : 'Login failed');
            throw error;
        } finally {
            userStore.setLoading(false);
        }
    }

    async signup(credentials: SignupCredentials): Promise<void> {
        try {
            userStore.setLoading(true);
            const response = await fetch(`${this.baseUrl}/auth/signup`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(credentials),
                credentials: 'include',
            });

            const data = await response.json();

            if (!response.ok) {
                throw new Error(data.message || 'Signup failed');
            }

            return data;
        } catch (error) {
            userStore.setError(error instanceof Error ? error.message : 'Signup failed');
            throw error;
        } finally {
            userStore.setLoading(false);
        }
    }

    async logout(): Promise<void> {
        try {
            userStore.setLoading(true);
            await fetch(`${this.baseUrl}/auth/logout`, {
                method: 'POST',
                credentials: 'include',
            });
            userStore.logout();
        } catch (error) {
            userStore.setError(error instanceof Error ? error.message : 'Logout failed');
            throw error;
        } finally {
            userStore.setLoading(false);
        }
    }

    async checkAuth(): Promise<void> {
        try {
            userStore.setLoading(true);
            console.log('Checking authentication status');

            const response = await fetch(`${this.baseUrl}/users/me`, {
                credentials: 'include',
            });

            console.log('Auth check response status:', response.status);
            if (!response.ok) {
                console.log('Auth check failed - not authenticated');
                throw new Error('Not authenticated');
            }

            const data = await response.json();
            console.log('Auth check response data:', data);

            userStore.setUser(data.user);
            console.log('User set in store after auth check:', data.user);
        } catch (error) {
            console.error('Auth check error:', error);
            userStore.logout();
            throw error;
        } finally {
            userStore.setLoading(false);
        }
    }

    async refreshToken(): Promise<void> {
        try {
            console.log('Attempting to refresh token');
            const response = await fetch(`${this.baseUrl}/auth/refresh`, {
                method: 'POST',
                credentials: 'include',
            });

            console.log('Token refresh response status:', response.status);
            if (!response.ok) {
                throw new Error('Token refresh failed');
            }

            await this.checkAuth(); // Re-fetch user data after token refresh
            console.log('Token refresh successful');
        } catch (error) {
            console.error('Token refresh error:', error);
            userStore.logout();
            throw error;
        }
    }
}

export const authService = new AuthService(); 