<script lang="ts">
    import { AuthenticationStatus } from '../stores/user.ts';
    import { removeAuthInfo } from '../api_wrapper/common/store_auth_info_cookie';
    import { onDestroy } from 'svelte';

    let user;
    const unsubscribe = AuthenticationStatus.subscribe((status) => {
        if (status.info) {
            user = status.info.user;
        } else {
            user = null;
        }
    });

    onDestroy(() => {
        unsubscribe();
    });

    function logoutUser() {
        removeAuthInfo();
        AuthenticationStatus.set({
            info: null
        });
    }
</script>

<div class="account-links flex justify-end">
    {#if user}
        <div class="user-dropdown-container flex justify-center items-center gap-5">
            <a href={`/u/${user.username}`}>
                <span class="link">
                    {user.username}
                </span>
            </a>
            {#if user.is_admin}
                <a href={`/admin`}>
                    <span class="link"> Manage NerdTree </span>
                </a>
            {/if}
            <a href={`/blogs/new`}>
                <span class="link"> Create Blog </span>
            </a>
            <button class="font-semibold cursor-pointer" on:click={() => logoutUser()}
                >Logout</button
            >
        </div>
    {:else}
        <div class="flex gap-3">
            <a href="/login">
                <span class="link">Log In</span>
            </a>
            <span class="font-extralight p-2">or</span>
            <a href="/register">
                <span class="link" id="register"> Register </span>
            </a>
        </div>
    {/if}
</div>

<style lang="scss">
    .account-links {
        span {
            padding: 0.2em 0.5em 0.2em 0.5em;
        }

        .link {
            font-size: 1em;
            font-weight: 600;
            letter-spacing: -0.5px;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            user-select: none;
        }

        #register {
            background: linear-gradient(95.48deg, #28292c 0%, #131415 100%);
            border: 1px solid rgba(255, 255, 255, 0.12);
            border-radius: 5px;
        }
    }

    .user-dropdown-container {
        display: flex;
        flex-wrap: wrap;
    }

    button {
        background: none;
    }
</style>
