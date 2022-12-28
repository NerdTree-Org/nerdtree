<script lang="ts">
    import type { User } from '../../interfaces/user';
    import { API } from '../../api_wrapper';
    import DefaultAvatar from '../../images/default-avatar.png';
    import { ENV } from '../../env.js';
    import { onDestroy, onMount } from 'svelte';
    import { AuthenticationStatus } from '../../stores/user';
    import { goto } from '$app/navigation';
    import { browser } from '$app/environment';
    import type { Post } from '../../interfaces/post';
    import BlogCard from '../../components/blog_card.svelte';
    import PaginationBar from '../../components/pagination_bar.svelte';
    import { getAccessToken } from '../../api_wrapper/common/store_auth_info_cookie';

    let user_search_result: User | null = null;

    let unsubscribe = () => {
        /**/
    };

    onDestroy(() => {
        unsubscribe();
    });

    let current_page = 1;
    let max_page = 1;
    let posts: Post[] = [];

    async function loadPosts() {
        const result = await API.post.query.unapproved.paginate({
            per_page: 10,
            page: current_page
        });

        if (result.success) {
            posts = result.value.page;
            max_page = result.value.max_page;
        }
    }

    async function updateApproval(id: string, state: boolean) {
        const result = await API.post.update.update_approval({
            approval_state: state,
            post_id: id
        });

        if (result.success) {
            await loadPosts();
        } else {
            alert(`Failed to change approval status: ${result.error}`);
        }
    }

    onMount(async () => {
        unsubscribe = AuthenticationStatus.subscribe((status) => {
            if (!browser) return;

            if (!status.info && typeof getAccessToken() !== 'undefined') {
                return;
            }

            if (!status.info) {
                goto('/login');
                return;
            }

            if (!status.info.user.is_admin) {
                goto('/');
                return;
            }
        });
        await loadPosts();
    });

    async function searchUser() {
        const query = document.querySelector('#search-input').value;
        const result = await API.user.query.username({
            username: query
        });

        if (result.success) {
            user_search_result = result.value;
        } else {
            user_search_result = null;
        }
    }

    async function deleteUser() {
        let confirmation = confirm(
            'Are you sure want to delete this user?\nTHIS ACTION CANNOT BE UNDONE!'
        );
        if (confirmation) {
            const result = await API.user.delete.user({
                user_id: user_search_result.id
            });

            if (result.success) {
                alert('Successfully deleted the user');
                await searchUser();
            } else {
                alert(`Failed to delete the user: ${result.error}`);
            }
        }
    }
</script>

<svelte:head>
    <title>Manage NerdTree</title>
</svelte:head>

<div class="top-container p-5">
    <h1>Manage NerdTree</h1>
    <div class="flex gap-5 flex-wrap mt-5">
        <div>
            <h2>Delete User</h2>
            <div class="users-list p-5">
                <input
                    id="search-input"
                    on:change={searchUser}
                    on:keyup={searchUser}
                    placeholder="Enter username here"
                />
                {#if user_search_result}
                    <div class="user-info">
                        <img
                            alt={`${user_search_result.username}'s profile picture`}
                            src={user_search_result.profile_pic
                                ? `${ENV.api_address}/static/${user_search_result.profile_pic}`
                                : DefaultAvatar}
                        />
                        <a href={`/u/${user_search_result.username}`}
                            ><span>u/{user_search_result.username}</span></a
                        >
                        <button on:click={deleteUser}>Remove</button>
                    </div>
                {/if}
            </div>
        </div>
        <div class="flex flex-col gap-10">
            <h2>Manage Posts</h2>
            <div class="flex flex-wrap gap-10">
                {#each posts as post}
                    <div class="blog-card-container">
                        <button
                            class="approval"
                            on:click={() => updateApproval(post.id, !post.is_approved)}
                            >{post.is_approved ? 'Disapprove' : 'Approve'}</button
                        >
                        <BlogCard
                            blog_id={post.id}
                            blog_author={post.post_author}
                            blog_post_time={new Date(post.creation_date)}
                            blog_title={post.title}
                            blog_image={post.thumbnail}
                        />
                    </div>
                {/each}
            </div>
            <div class="p-10 flex justify-center">
                <PaginationBar
                    {current_page}
                    {max_page}
                    callback={(page) => {
                        current_page = page;
                        loadPosts();
                    }}
                />
            </div>
        </div>
    </div>
</div>

<style lang="scss">
    .top-container {
        min-height: 90vh;
    }

    .users-list {
        background: linear-gradient(180deg, #484848 0%, rgba(72, 72, 72, 0.12) 100%);
        border-radius: 13px;

        input {
            width: 100%;
            padding: 0.6em 1em 0.6em 1em;
            margin-bottom: 0.5em;
            background: #282828;
            border: 1px solid #505050;
            font-weight: 600;
            letter-spacing: -0.025em;
            border-radius: 9px;
            font-size: 18px;
            outline: none;
            transition: 200ms;
        }

        .user-info {
            display: flex;
            justify-content: space-evenly;
            align-items: center;
            padding: 0.5em;
            background: #232323;
            border-radius: 10px;
            font-weight: 600;
            border: 1px solid #505050;

            img {
                width: 45px;
                border-radius: 50%;
            }

            button {
                color: #ff4f4f;
            }
        }
    }

    .blog-card-container {
        max-width: min(400px, 100%);
    }

    .approval {
        padding: 1em;
        font-weight: 600;
    }
</style>
