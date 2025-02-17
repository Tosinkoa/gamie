## SpecStory

## Profile Page Access Issue (17/02/2025, 15:17:45)

_**User**_

After logging in, I was not able to access profile page, but rather get redirected back to login page. Help fix this.


@RedirectGuard.svelte @AuthGuard.svelte @+page.svelte @+page.svelte 

---

_**Assistant**_

```diff
      try {
        await authService.checkAuth();
-       if (!$isAuthenticated) {
-         goto(redirectTo);
-       }
      } catch (error) {
-       goto(redirectTo);
+       // Optionally log the error
      } finally {
        isLoading = false;
      }
    });
+ 
+   $: if (!isLoading && !$isAuthenticated) {
+     goto(redirectTo);
+   }
  </script>
  
```

---

_****_

```diff
    });
  
-   $: if (!isLoading && !$isAuthenticated) {
+   $: if (!isLoading && !$userStore.user) {
      goto(redirectTo);
    }
```

---

_****_

```diff
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
-   import { isAuthenticated } from "$lib/stores/user";
+   import { isAuthenticated, userStore } from "$lib/stores/user";
    import { authService } from "$lib/services/auth";
  
      try {
        await authService.checkAuth();
-       if ($isAuthenticated) {
+       if ($userStore.user) {
          goto(redirectTo);
        }
  </script>
  
- {#if !$isAuthenticated}
+ {#if !$userStore.user}
    {#if isLoading}
      <div
```

---

_****_

The issue was that the guards were relying on the $isAuthenticated flag rather than checking whether userStore.user was set, so even after a successful login the guard did not recognize the user. I’ve updated both the AuthGuard.svelte and RedirectGuard.svelte components to check for the presence of a user (using $userStore.user) and added a reactive check in AuthGuard. This should now allow authenticated users (i.e. those with a user object in the store) to access the profile page rather than getting redirected back to login.

Please try reloading the profile page after logging in to verify that the fix works as expected.

---

_**User**_

@user.ts 
I am not sure why this error still persist. Still getting redirected to login page after logging in. 

---

_**Assistant**_

```diff
  
  function createUserStore() {
-     const { subscribe, set, update } = writable<UserState>(initialState);
+     let initial: UserState = initialState;
+     if (browser) {
+         const storedUser = localStorage.getItem('user');
+         if (storedUser) {
+             try {
+                 initial = { ...initialState, user: JSON.parse(storedUser) };
+             } catch (err) {
+                 // ignore parsing errors
+             }
+         }
+     }
+     const { subscribe, set, update } = writable<UserState>(initial);
  
      return {
          subscribe,
-         setUser: (user: User | null) => update(state => ({ ...state, user, loading: false })),
+         setUser: (user: User | null) => {
+             if (browser) {
+                 if (user) {
+                     localStorage.setItem('user', JSON.stringify(user));
+                 } else {
+                     localStorage.removeItem('user');
+                 }
+             }
+             update(state => ({ ...state, user, loading: false }));
+         },
          setError: (error: string) => update(state => ({ ...state, error, loading: false })),
          setLoading: (loading: boolean) => update(state => ({ ...state, loading })),
```

---

_****_

```diff
      try {
        await authService.checkAuth();
+       if (!$userStore.user) {
+         goto(redirectTo);
+       }
      } catch (error) {
-       // Optionally log the error
+       goto(redirectTo);
      } finally {
        isLoading = false;
```

---

_****_



---

_****_



---

_**User**_

But its bad to add JSON web token inside localstorage.
@main.rs @handlers.rs 
```

To reiterate, whatever you do, don't store a JWT in local storage (or session storage). If any of the third-party scripts you include in your page is compromised, it can access all your users' tokens. To keep them secure, you should always store JWTs inside an httpOnly cookie.
```

---

_**Assistant**_



---

_****_



---



---

