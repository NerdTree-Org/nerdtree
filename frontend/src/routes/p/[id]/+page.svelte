<script lang="ts">
    import { page } from '$app/stores';
    import { API } from '../../../api_wrapper';
    import type { Post } from '../../../interfaces/post';
    import { onMount } from 'svelte';
    import { ENV } from '../../../env';
    import type { User } from '../../../interfaces/user';
    import type { Comment } from '../../../interfaces/comment';
    import MD from 'markdown-it';
    import xss from 'xss';
    import { AuthenticationStatus } from '../../../stores/user.js';
    import NerdTreeButton, { ButtonType } from '../../../components/nerdtree_button.svelte';
    import hljs from 'highlight.js';
    import 'highlight.js/styles/github-dark.css';
    import { get } from 'svelte/store';
    import { browser } from '$app/environment';
    import CommentComponent from '../../../components/comment.svelte';
    import PaginationBar from '../../../components/pagination_bar.svelte';

    $: post_id = $page.params.id;
    export let data;
    let post: Post | null = data.post;
    let post_author: User | null = data.post_author;
    let is_unapproved = false;
    let post_body_ref = -1;
    let comments: Comment[] = [];
    let current_comments_page = 1;
    let max_pages = 1;
    let post_votes = 0;
    let has_upvoted = false;
    let has_downvoted = false;

    $: fetchPost();

    async function fetchPost() {
        const result = await API.post.query.id({
            post_id: $page.params.id,
        });

        if (result.success) {
            post = result.value;
            post.body = xss(
                MD({ html: true, whitelist: { a: ['href', 'target'] } }).render(result.value.body)
            );
            const result2 = await API.user.query.id({
                id: result.value.post_author
            });

            if (result2.success) {
                post_author = result2.value;
                await fetchCommentsAndUpvotes();
            }
        }
    }
    fetchPost();

    async function submitComment() {
        const body = document.querySelector<HTMLTextAreaElement>('#comment-container').value;
        if (body.length == 0) return;

        const result = await API.comment.update.new({
            body,
            post_id
        });

        if (result.success) {
            await fetchCommentsAndUpvotes();
            document.querySelector<HTMLTextAreaElement>('#comment-container').value = '';
        }
    }

    async function upvote() {
        const result = await API.post.vote.upvote({
            post_id
        });

        if (result.success) {
            await fetchCommentsAndUpvotes();
        }
    }

    async function downvote() {
        const result = await API.post.vote.downvote({
            post_id
        });

        if (result.success) {
            await fetchCommentsAndUpvotes();
        }
    }

    async function fetchCommentsAndUpvotes() {
        {
            const result = await API.comment.query.by_post({
                post_id,
                page: current_comments_page,
                per_page: 10
            });

            if (result.success) {
                comments = result.value.page;
                max_pages = result.value.max_page;
            }
        }

        {
            {
                const result = await API.post.vote.votes({
                    post_id
                });

                if (result.success) {
                    post_votes = result.value.votes;
                }
            }
            {
                if (!browser) {
                    return;
                }
                if (get(AuthenticationStatus).info) {
                    const result = await API.post.vote.by_current_user({
                        post_id
                    });

                    if (result.success) {
                        let vote = result.value.vote;
                        if (vote < 0) {
                            has_upvoted = false;
                            has_downvoted = true;
                        } else if (vote > 0) {
                            has_upvoted = true;
                            has_downvoted = false;
                        } else {
                            has_upvoted = false;
                            has_downvoted = false;
                        }
                    }
                }
            }
        }
    }

    onMount(async () => {
        if (!post) {
            const result = await API.post.query.unapproved.id({
                post_id
            });

            if (result.success) {
                await fetchCommentsAndUpvotes();
                is_unapproved = true;
                post = result.value;
                post.body = xss(MD({ html: true }).render(post.body));

                const result2 = await API.user.query.id({
                    id: post.post_author
                });

                if (result2.success) {
                    post_author = result2.value;
                }
            }
        }

        document.querySelectorAll('pre').forEach((el) => {
            hljs.highlightElement(el);
        });
    });
</script>

<svelte:head>
    <title>{post ? post.title : 'Not Found'} - NerdTree</title>
</svelte:head>

<div class="top-container p-10 flex flex-col items-center justify-center">
    {#if post}
        <div class="post-container">
            <div class="post-head">
                <img
                    alt="post-thumbnail"
                    src={post.thumbnail
                        ? `${ENV.api_address}/static/${post.thumbnail}`
                        : 'https://via.placeholder.com/600x400'}
                />
                <div class="post-head-info flex flex-col gap-2">
                    <h1>{post.title}</h1>
                    <a href={`/u/${post_author ? post_author.username : 'deleted_user'}`}>
                        <p>by u/{post_author ? post_author.username : 'deleted_user'}</p>
                    </a>
                    <p>Posted on: {new Date(post.creation_date).toDateString()}</p>
                    <p>Votes: {post_votes}</p>
                    {#if $AuthenticationStatus.info}
                        <div>
                            <button
                                class={`p-1 rounded-md ${has_upvoted && 'border-2'}`}
                                on:click={upvote}>Upvote</button
                            >
                            <button
                                class={`p-1 rounded-md ${has_downvoted && 'border-2'}`}
                                on:click={downvote}>Downvote</button
                            >
                        </div>
                    {/if}
                    {#if post.post_author === $AuthenticationStatus.info?.user.id}
                        <a href={`/p/${post.id}/edit`}>
                            <NerdTreeButton type={ButtonType.Smooth}>Edit</NerdTreeButton>
                        </a>
                    {/if}
                </div>
            </div>
            <div bind:this={post_body_ref} class="post-body">
                {@html post.body}
                {#if false}
                    <img
                        alt="blahblah"
                        src="https://imgs.xkcd.com/comics/particle_properties.png"
                    />
                    <p>Woah, looks like a cosmic particle has hit your computer</p>
                    <a href="https://xkcd.com/1862/">Here's a comic for you</a>
                {/if}
            </div>
            <div class="comments flex flex-col gap-5 mt-5">
                <h2>Comments</h2>
                {#if $AuthenticationStatus.info && post.is_approved}
                    <div class="flex flex-col items-end gap-5">
                        <textarea id="comment-container" placeholder="Type your comment here" />
                        <NerdTreeButton on_click={submitComment} type={ButtonType.Smooth}
                            >Submit</NerdTreeButton
                        >
                    </div>
                {/if}
                {#each comments as comment}
                    <CommentComponent
                        author_id={comment.author_id}
                        body={comment.body}
                        creation_date={new Date(comment.creation_date)}
                    />
                {/each}
                {#if comments.length === 0}
                    <p>No comments available</p>
                {:else}
                    <div class="flex justify-center">
                        <PaginationBar
                            callback={(page) => {
                                current_comments_page = page;
                                fetchCommentsAndUpvotes();
                            }}
                            current_page={current_comments_page}
                            max_page={max_pages}
                        />
                    </div>
                {/if}
            </div>
        </div>
    {:else}
        <h1>Post doesn't exist</h1>
    {/if}
</div>

<style lang="scss">
    .top-container {
        min-height: 87vh;
    }

    .post-container {
        max-width: min(700px, 100%);
        font-weight: 600;
    }

    .post-head {
        display: flex;
        justify-content: center;
        flex-wrap: wrap;
        gap: 2em;
        margin-bottom: 3em;

        .post-head-info {
            max-width: 100%;
        }

        h1 {
            overflow-wrap: break-word;
        }

        img {
            width: min(100%, 350px);
            border-radius: 20px;
        }

        p {
            font-size: 1.2em;
            font-weight: 600;
            letter-spacing: -0.025em;
            color: #686868;
        }
    }

    .post-body {
        display: flex;
        flex-direction: column;
        gap: 2em;

        :global(p) {
            display: flex;
            flex-direction: column;
            overflow-wrap: break-word;
            gap: 2em;
        }
        :global(img) {
            border-radius: 20px;
        }

        :global(a) {
            color: #539cff;
            transition: 200ms;

            &:hover {
                color: #90beff;
            }
        }

        :global(td),
        :global(th) {
            padding: 1em;
            border: 1px solid #505050;
        }
        :global(pre) {
            overflow-x: auto;
        }
    }

    .comments {
        textarea {
            width: 100%;
            outline: none;
            padding: 1em 0;
            border-bottom: 2px solid #696969;
            min-height: calc(24px + 3em);
            color: #696969;
            background: none;
            font-size: 24px;
        }
    }

    :global(.hljs) {
        padding: 2em;
        border-radius: 10px;
        font-family: 'JetBrainsMono', monospace;
        font-weight: 600;
    }
</style>
