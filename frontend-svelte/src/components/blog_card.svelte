<script lang="ts">
    import { User } from '../interfaces/user';

    export let blog_id: string;
    export let blog_title: string;
    export let blog_image: string;
    export let blog_author: User | null;
    export let blog_votes: number;

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
    <a href={`/u/${$blog_author ? $blog_author.username : 'deleted_user'}`}>
        <h3>
            u/{$blog_author ? $blog_author.username : 'deleted_user'}
        </h3>
    </a>
    <a href={`/blogs/${$blog_id}`}>
        <h1>{$blog_title}</h1>
    </a>
    <h4>{`${round_number($blog_votes)} upvotes`}</h4>
    <img src={$blog_image} alt={'Blog Thumbnail'} />
</div>

<style lang="scss">
    .blog-card {
        max-width: 400px;
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
