<script lang="ts">
    import type { Post } from '../../interfaces/post';
    import { API } from '../../api_wrapper';
    import BlogCard from '../../components/blog_card.svelte';
    import { onMount } from 'svelte';
    import PaginationBar from '../../components/pagination_bar.svelte';

    let posts: Post[] = [];
    let error: string | null = null;
    let max_page = 1;
    let current_page = 1;
    let is_loading = false;

    async function loadPosts(perPage: number, page: number) {
        is_loading = true;
        error = null;
        const result = await API.post.query.paginate({
            per_page: perPage,
            page
        });

        if (result.success) {
            posts = result.value.page;
            max_page = result.value.max_page;
            current_page = result.value.current_page;
        } else {
            error = result.error;
        }
        is_loading = false;
    }

    onMount(async () => {
        await loadPosts(10, 1);
    });
</script>

<svelte:head>
    <title>Blogs - NerdTree</title>
</svelte:head>

<div class="min-h-screen flex flex-col">
    <h1 class="text-center block p-10">Blogs</h1>
    {#if is_loading}
        <div class="flex justify-center items-center">
            <div class="spinner" />
        </div>
    {/if}
    {#if error}
        <h2>{error}</h2>
    {:else}
        <div class="p-10 flex flex-wrap gap-10">
            {#each posts as post}
                <BlogCard
                    blog_id={post.id}
                    blog_author={post.post_author}
                    blog_post_time={new Date(post.creation_date)}
                    blog_title={post.title}
                    blog_image={post.thumbnail}
                />
            {/each}
        </div>
        <div class="p-10 flex justify-center">
            <PaginationBar
                {current_page}
                {max_page}
                callback={(page) => {
                    loadPosts(10, page);
                }}
            />
        </div>
    {/if}
</div>

<style lang="scss">
    .spinner {
        height: 50px;
        width: 50px;
        border: 3px solid #303030;
        border-radius: 50%;
        border-top-color: #606060;
        animation: spin 1s infinite linear;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>
