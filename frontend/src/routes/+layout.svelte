<script lang="ts">
    import '../styles/app.scss';
    import { get, writable } from 'svelte/store';
    import { onMount, setContext } from 'svelte';
    import Header from '../components/header.svelte';
    import Footer from '../components/footer.svelte';
    import {
        getAccessToken,
        getLastLogin,
        getRefreshToken,
        removeAuthInfo
    } from '../api_wrapper/common/store_auth_info_cookie';
    import { API } from '../api_wrapper';
    import { AuthenticationStatus } from '../stores/user';
    import { getCurrentUser } from '../api_wrapper/user/query';
    import { page } from '$app/stores';

    const excludeHeaders = ['/', '/login', '/register', '/password_reset', '/reset'];
    const layout = writable('default_layout');
    setContext('setLayout', layout.set);

    onMount(() => {
        // refresh accesstoken every 5 seconds if the user is logged in
        async function refresh_accesstoken() {
            const refresh_token = getRefreshToken();
            if (typeof refresh_token === 'undefined') {
                if (get(AuthenticationStatus).info) {
                    AuthenticationStatus.set({
                        info: null
                    });
                }

                return;
            }
            let result = await API.auth.refresh_token({
                refresh_token
            });

            if (!result.success) {
                // remove these so that we don't make unnecessary fetch requests
                // when the authentication info is no longer valid
                removeAuthInfo();
                AuthenticationStatus.set({
                    info: null
                });
            } else {
                // update the user information
                const current_user = await getCurrentUser();

                AuthenticationStatus.set({
                    info: {
                        user: current_user.value,
                        access_token: getAccessToken(),
                        refresh_token: getRefreshToken(),
                        last_login: getLastLogin()
                    }
                });
            }
        }

        const interval = setInterval(refresh_accesstoken, 5000);
        refresh_accesstoken();

        return () => clearInterval(interval);
    });
</script>

{#if !excludeHeaders.includes('/' + $page.url.pathname.split('/').pop())}
    <Header />
    <slot />
    <Footer />
{:else}
    <slot />
{/if}
