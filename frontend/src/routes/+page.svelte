<script lang="ts">
    import AuthStatus from '../components/auth_status.svelte';
    import HorizontalScroll from '../components/horizontal_scroll.svelte';
    import RepoCard from '../components/repo_card.svelte';
    import Footer from '../components/footer.svelte';
    import Logo from '../images/logo.png';
    import { API } from '../api_wrapper';
    import type { Post } from '../interfaces/post';
    import { onMount } from 'svelte';
    import BlogCard from '../components/blog_card.svelte';

    let posts: Post[] = [];
    async function loadBlogs() {
        const result = await API.post.query.paginate({
            page: 1,
            per_page: 5
        });

        if (result.success) {
            posts = result.value.page;
        }
    }

    onMount(async () => {
        await loadBlogs();
    });
</script>

<svelte:head>
    <title>NerdTree</title>
</svelte:head>

<div>
    <div class="toplevel flex flex-col items-center min-w-screen min-h-screen justify-between">
        <div class="flex justify-end p-4 w-screen">
            <AuthStatus />
        </div>
        <div class="greeting-container w-full">
            <div>
                <h1>We&apos;re NerdTree</h1>
                <h2>An organization aiming to make a better programming community for students</h2>
            </div>
            <img src={Logo} alt="NerdTree Logo" />
        </div>
        <div class="nav-links-container">
            <button id="index">/</button>
            <a href="/blogs">
                <span>Blogs</span>
            </a>
            <a href="/about">
                <span>About us</span>
            </a>
        </div>
    </div>
    <div class="p-5">
        <h2>Latest Blogs</h2>
        <HorizontalScroll>
            {#each posts as post}
                <BlogCard
                    blog_author={post.post_author}
                    blog_title={post.title}
                    blog_post_time={new Date(post.creation_date)}
                    blog_id={post.id}
                    blog_image={post.thumbnail}
                />
            {/each}
        </HorizontalScroll>
    </div>
    <div class="p-5">
        <h2>Github Projects</h2>
        <HorizontalScroll>
            <RepoCard
                link={'https://github.com/NerdTree-Org/NerdTree'}
                issues={'https://github.com/NerdTree-Org/NerdTree/issues'}
                maintainers={['mdgaziur001']}
                repo={'NerdTree-Org/NerdTree'}
            />
            <RepoCard
                link={'https://github.com/NerdTree-Org/NerdTree-Wishlist'}
                issues={'https://github.com/NerdTree-Org/NerdTree-Wishlist/issues'}
                maintainers={['mdgaziur001']}
                repo={'NerdTree-Org/NerdTree-Wishlist'}
            />
        </HorizontalScroll>
    </div>
    <Footer />
</div>

<style lang="scss">
    .toplevel {
        background: radial-gradient(
            67.11% 67.11% at 47.48% 18.31%,
            #333333 0%,
            rgba(36, 36, 36, 0) 100%
        );
    }

    .greeting-container {
        padding: 2.5rem;
        gap: 50px;
        display: flex;
        align-content: center;
        justify-content: space-around;
        flex-wrap: wrap-reverse;

        div {
            display: flex;
            align-items: flex-start;
            justify-content: center;
            flex-direction: column;

            h1 {
                display: block;
            }

            h2 {
                display: block;
                font-size: 25px;
                max-width: 400px;
            }
        }

        img {
            border-radius: 100%;
        }
    }

    .nav-links-container {
        background: #343434;
        border: 1px solid rgba(255, 255, 255, 0.24);
        border-radius: 13px;
        padding: 0.5em;
        margin: 1em;
        display: flex;
        flex-wrap: wrap;
        gap: 10px;

        button {
            padding: 0.5em 1em 0.5em 1em;
            background: #646464;
            border: 1px solid rgba(255, 255, 255, 0.16);
            border-radius: 8px;
            text-align: left;
        }

        a {
            padding: 0.5em 1em 0.5em 1em;
            font-weight: 600;
            font-size: 100%;
            cursor: pointer;
            user-select: none;
            background-color: transparent;
            border-radius: 8px;
            transition: 200ms;

            &:hover {
                background: #101010;
            }
        }
    }

    @media only screen and (max-width: 423px) {
        .nav-links-container {
            flex-direction: column;
            width: 95%;
        }
    }
</style>
