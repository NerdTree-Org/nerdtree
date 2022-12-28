<script lang="ts">
    import { page } from '$app/stores';
    import type { User } from '../../../interfaces/user';
    import { onDestroy, onMount } from 'svelte';
    import { API } from '../../../api_wrapper';
    import DefaultAvatar from '../../../images/default-avatar.png';
    import { ENV } from '../../../env.js';
    import { ButtonType } from '../../../components/nerdtree_button.svelte';
    import NerdTreeButton from '../../../components/nerdtree_button.svelte';
    import { AuthenticationStatus } from '../../../stores/user';
    import type { Post } from '../../../interfaces/post';
    import BlogCard from '../../../components/blog_card.svelte';
    import { get } from 'svelte/store';

    $: username = $page.params.username;
    export let data;
    let user: User | null = data.user;
    let currentUser: User | null;
    let posts: Post[] = data.posts;

    const unsubscribe = AuthenticationStatus.subscribe((value) => {
        if (value.info) {
            currentUser = value.info.user;
        } else {
            currentUser = null;
        }
    });

    onDestroy(() => {
        unsubscribe();
    });

    $: {
        (async () => {
            const result = await API.user.query.username({
                username
            });

            if (result.success) {
                user = result.value;

                let result2;

                if (get(AuthenticationStatus).info?.user.id == user.id) {
                    result2 = await API.post.query.unapproved.author_id({
                        author_id: user.id
                    });
                } else {
                    result2 = await API.post.query.author_id({
                        author_id: user.id
                    });
                }

                if (result2.success) {
                    posts = result2.value;
                }
            }
        })();
    }
</script>

<svelte:head>
    {#if !user}
        <title>NerdTree</title>
    {:else}
        <title>{user.username} at NerdTree</title>
    {/if}
</svelte:head>

<div class="info-container w-screen">
    {#if !user}
        <h1 class="text-center block">User not found!</h1>
    {:else}
        <div class="user-info-top items-center flex flex-wrap gap-20 p-10">
            <img
                src={user.profile_pic
                    ? `${ENV.api_address}/static/${user.profile_pic}`
                    : DefaultAvatar}
                alt={`${user.username}'s profile picture`}
            />
            <div class="flex flex-1 flex-wrap items-center">
                <div class="flex-1">
                    <h1>
                        {user.firstname + ' ' + user.lastname}
                        {#if user.is_admin}
                            <sup class="text-xl">Root</sup>
                        {/if}
                    </h1>
                    <h3 class="block">u/{user.username}</h3>
                </div>
                <div>
                    <h3 class="block mb-5">Joined: {new Date(user.joined_date).toDateString()}</h3>
                    {#if currentUser && currentUser.id === user.id}
                        <a href="/settings">
                            <NerdTreeButton type={ButtonType.Smooth}
                                >Account settings</NerdTreeButton
                            >
                        </a>
                    {/if}
                </div>
            </div>
        </div>
        {#if posts?.length > 0}
            <h1 class="block text-center p-5">Posts by this user</h1>
        {/if}
        <div class="flex flex-wrap gap-2 w-screen p-2 justify-start">
            {#if posts?.length > 0}
                {#each posts as post}
                    <BlogCard
                        blog_author={post.post_author}
                        blog_id={post.id}
                        blog_image={post.thumbnail}
                        blog_post_time={new Date(post.creation_date)}
                        blog_title={post.title}
                    />
                {/each}
            {:else}
                <h1 class="flex-1 block text-center">No posts were made by this user</h1>
            {/if}
        </div>
    {/if}
</div>

<style lang="scss">
    .info-container {
        min-height: 90vh;
    }
    img {
        max-width: 150px;
        border-radius: 50%;
    }
    .user-info-top {
        background: linear-gradient(358.11deg, #181818 1.82%, rgba(24, 24, 24, 0) 98.63%);
        color: #595959;
    }
</style>
