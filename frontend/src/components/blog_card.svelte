<script lang="ts">
    import type { User } from '../interfaces/user';
    import { onMount } from 'svelte';
    import { API } from '../api_wrapper';
    import { ENV } from '../env';

    export let blog_id: string;
    export let blog_title: string;
    export let blog_post_time: Date;
    export let blog_image: string | null;
    export let blog_author: string | null;

    let blog_votes = 0;
    let author: User | null = null;

    onMount(async () => {
        {
            const result = await API.user.query.id({
                id: blog_author
            });

            if (result.success) {
                author = result.value;
            }
        }

        {
            const result = await API.post.vote.votes({
                post_id: blog_id
            });

            if (result.success) {
                blog_votes = result.value.votes;
            }
        }
    });

    function round_number(n: number): string {
        let abs_n = Math.abs(n);
        let result = n < 0 ? '-' : '';

        if (abs_n / 1_000_000 >= 1) {
            result += `${(abs_n / 1_000_000).toFixed(1)}m`;
        } else if (abs_n / 1000 >= 1) {
            result += `${(abs_n / 1000).toFixed(1)}k`;
        } else {
            result += abs_n;
        }

        return result;
    }
</script>

<div class="blog-card">
    <a href={`/u/${author ? author.username : 'deleted_user'}`}>
        <h3>
            u/{author ? author.username : 'deleted_user'}
        </h3>
    </a>
    <a href={`/p/${blog_id}`}>
        <h1>{blog_title}</h1>
    </a>
    <h4>{blog_post_time.toDateString()}</h4>
    <h4>{`${round_number(blog_votes)} upvotes`}</h4>
    <img
        src={blog_image
            ? `${ENV.api_address}/static/${blog_image}`
            : 'https://via.placeholder.com/600x400'}
        alt={'Blog Thumbnail'}
    />
</div>

<style lang="scss">
    .blog-card {
        min-width: min(400px, 100%);
        background: linear-gradient(
            223.22deg,
            rgba(109, 109, 109, 0.42) -23.79%,
            rgba(255, 255, 255, 0) 100.29%
        );
        border: 1px solid rgba(255, 255, 255, 0.24);
        border-radius: 18px;
        padding: 1.5em;
        display: flex;
        flex-direction: column;
        flex-basis: 300px;
        flex-shrink: 0;
        gap: 5px;
        height: min-content;

        img {
            width: 100%;
            border-radius: 18px;
        }

        h1 {
            overflow-wrap: break-word;
        }

        h1,
        h3 {
            user-select: none;
            cursor: pointer;
        }

        h3 {
            color: #bdbdbd;
        }
    }
</style>
