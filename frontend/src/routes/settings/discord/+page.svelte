<script lang="ts">
    import SideBar from '../../../components/user_settings_sidebar.svelte';
    import { onDestroy, onMount } from 'svelte';
    import { AuthenticationStatus } from '../../../stores/user';
    import { getAccessToken } from '../../../api_wrapper/common/store_auth_info_cookie';
    import { goto } from '$app/navigation';
    import { API } from '../../../api_wrapper';

    let token = '';
    let unsubscribe: () => void = () => {
        /**/
    };
    onMount(async () => {
        unsubscribe = AuthenticationStatus.subscribe((status) => {
            if (!status.info && typeof getAccessToken() === 'undefined') {
                goto('/');
            }
        });

        const result = await API.user.get.discord_token();
        if (result.success) {
            token = result.value;
        } else {
            alert(`Failed to get discord token: ${result.error}`);
        }
    });

    onDestroy(() => {
        unsubscribe();
    });
</script>

<svelte:head>
    <title>Leave Nerdtree - NerdTree</title>
</svelte:head>

<div class="top-container flex flex-wrap">
    <SideBar idx="1" />
    <div class="flex gap-5 flex-col p-10 flex-1 max-w-full">
        <h2>Discord token: <code>{token}</code></h2>
        <p>Note: this code is usable only once. Contact root nodes aka admins if you encounter any issue.</p>
    </div>
</div>

<style lang="scss">
    .top-container {
        min-height: 87vh;
    }

    p {
        font-weight: 600;
        font-size: 20px;
    }

    @media only screen and (min-width: 650px) {
        .root-nodes-list {
            @apply pl-5;
        }
    }
</style>
