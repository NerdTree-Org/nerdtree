<script lang="ts">
    import SideBar from '../../../components/user_settings_sidebar.svelte';
    import {onDestroy, onMount} from "svelte";
    import {AuthenticationStatus} from "../../../stores/user";
    import {getAccessToken} from "../../../api_wrapper/common/store_auth_info_cookie";
    import {goto} from "$app/navigation";

    let unsubscribe: () => void = () => {/**/};
    onMount(() => {
        unsubscribe = AuthenticationStatus.subscribe((status) => {
            if (!status.info && typeof getAccessToken() === "undefined") {
                goto("/");
            }
        });
    });

    onDestroy(() => {
        unsubscribe();
    });
</script>

<svelte:head>
    <title>Leave Nerdtree - NerdTree</title>
</svelte:head>

<div class="top-container flex flex-wrap">
    <SideBar idx="2" />
    <div class="flex gap-5 flex-col p-10 flex-1 max-w-full">
        <p>
            Leaving NerdTree is *not* an automatic process. You have to contact one of the root
            nodes to request membership removal
        </p>
        <p>
            Root Nodes:
        </p>
        <div class="root-nodes-list">
            <p style="color: #969696;">u/n00b_shanto(SHANTO on Discord)</p>
            <p style="color: #969696;">u/mehedirm</p>
            <p style="color: #969696;">u/uthsob_cb</p>
            <p style="color: #969696;">u/jisangain</p>
            <p style="color: #969696;">u/mdgaziur001</p>
        </div>
        <p>
            DM one of the root nodes on Discord to request membership removal. Membership removal
            will be done within 24 hours in the best case scenario. Keep in mind that you have to be a
            member for at least 1 week before requesting membership removal.
        </p>
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