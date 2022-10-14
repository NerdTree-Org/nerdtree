<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import '@toast-ui/editor/dist/theme/toastui-editor-dark.css';
    import '@toast-ui/editor/dist/toastui-editor.css';
    import NerdTreeButton, { ButtonType } from '../../../../components/nerdtree_button.svelte';
    import { AuthenticationStatus } from '../../../../stores/user';
    import { goto } from '$app/navigation';
    import { browser } from '$app/environment';
    import { API } from '../../../../api_wrapper';
    import { getAccessToken } from '../../../../api_wrapper/common/store_auth_info_cookie';
    import { page } from '$app/stores';
    import type { Post } from '../../../../interfaces/post';
    import { ENV } from '../../../../env';

    $: post_id = $page.params.id;
    let createBlog: () => void = () => {
        /* to fix error about empty function body(webstorm) */
    };
    let post: Post | null = null;
    let error = '';
    let thumbnailError = '';
    let isSubmitting = false;

    function setError(err: string) {
        error = err;
        window.location.href = '#error-info';
    }

    let unsubscribe = () => {
        /**/
    };
    onDestroy(() => {
        unsubscribe();
    });
    onMount(async () => {
        {
            const result = await API.post.query.unapproved.id({
                post_id
            });

            if (result.success) {
                post = result.value;
            } else {
                return;
            }
        }

        unsubscribe = AuthenticationStatus.subscribe(async (status) => {
            if (!status.info && browser && typeof getAccessToken() === 'undefined') {
                await goto('/login');
            }
        });

        const Editor = (await import('@toast-ui/editor')).Editor;

        const editor = new Editor({
            el: document.querySelector('#editor'),
            previewStyle: 'vertical',
            height: '700px',
            theme: 'dark',
            initialEditType: 'wysiwyg'
        });
        editor.setMarkdown(post.body);

        createBlog = async () => {
            isSubmitting = true;
            const markdown = editor.getMarkdown().trim();
            const title = document.querySelector('#title').value.trim();
            if (title.length < 3 || title.length > 255) {
                setError('Max title length is 255 and min length is 3');
                isSubmitting = false;
                return;
            }
            if (markdown == '') {
                setError('Post body is required');
                isSubmitting = false;
                return;
            }

            {
                const result = await API.post.update.title({
                    new_title: title,
                    post_id
                });

                if (!result.success) {
                    alert('Failed to update blog title');
                }
            }

            {
                const result = await API.post.update.body({
                    body: markdown,
                    post_id
                });

                if (!result.success) {
                    alert('Failed to update blog body!');
                }
            }

            {
                let img_input = document.querySelector<HTMLInputElement>('#thumbnail-field');

                if (img_input.files && img_input.files[0]) {
                    let form_data = new FormData();
                    form_data.append('post_id', post_id);
                    form_data.append('thumbnail', img_input.files[0]);

                    const result2 = await API.post.update.thumbnail({
                        form_data
                    });

                    if (result2.success) {
                        alert('Successfully updated the thumbnail!');
                        await goto('/');
                    } else {
                        alert(`Failed to update the thumbnail: ${result2.error}`);
                    }
                }
            }
            alert('Successfully updated the post!');
            isSubmitting = false;
        };
    });

    function loadImage() {
        let input = document.querySelector<HTMLInputElement>('#thumbnail-field');

        if (input.files && input.files[0]) {
            if (input.files[0].size / (1024 * 1024) > 2) {
                thumbnailError = 'Maximum allowed thumbnail size is 2MiB';
                input.value = null;
                return;
            }
            thumbnailError = '';

            let reader = new FileReader();
            reader.onload = (e) => {
                let thumbnail_preview: HTMLImageElement =
                    document.querySelector('#thumbnail-preview');
                thumbnail_preview.setAttribute('src', <string>e.target.result);
            };
            reader.readAsDataURL(input.files[0]);
        }
    }
</script>

<svelte:head>
    <title>Edit blog - NerdTree</title>
</svelte:head>

<div class="container">
    {#if post}
        <h1>Edit Blog</h1>
        <div class="input-container">
            <label for="title">Title</label>
            <input id="title" name="Title" value={post.title} />
        </div>
        <div id="thumbnail-upload">
            <img
                id="thumbnail-preview"
                alt="Blog Thumbnail"
                src={`${ENV.api_address}/static/${
                    post.thumbnail ? post.thumbnail : 'https://via.placeholder.com/600x400'
                }`}
            />
            <div class="p-5">
                <div>
                    <label for="thumbnail-field">Upload Thumbnail</label>
                    <span class="note">Max size: 2MiB</span>
                    <span class="block text-red-700">{thumbnailError}</span>
                </div>
                <input
                    on:change={loadImage}
                    id="thumbnail-field"
                    name="Blog Thumbnail"
                    accept="image/png"
                    type="file"
                />
            </div>
        </div>
        <div class="mt-10" id="editor" />
        <div class="pt-10">
            <span class="block text-red-700 mb-5">{error}</span>
            <NerdTreeButton on_click={createBlog} type={ButtonType.Smooth}>
                {isSubmitting ? 'Updating...' : 'Update'}
            </NerdTreeButton>
        </div>
    {:else}
        <h1>Post doesn't exist</h1>
    {/if}
</div>

<style lang="scss">
    .container {
        padding: 10%;
    }

    .input-container {
        display: flex;
        flex-direction: column;
        gap: 1em;
        margin-top: 2em;

        label {
            font-size: 1.5em;
            color: #8d8d8d;
            font-weight: 600;
            letter-spacing: -0.025em;
        }

        input {
            outline: none;
            padding: 0.3em;
            background: none;
            font-weight: 600;
            font-size: 2.5em;
            letter-spacing: -0.025em;
            border-bottom: 5px solid #202020;
            transition: 200ms;
            font-family: 'Poppins', sans-serif;

            &:focus {
                border-bottom: 5px solid #303030;
            }
        }
    }

    #thumbnail-upload {
        margin-top: 2em;
        display: flex;
        flex-wrap: wrap;
        gap: 1em;

        input {
            display: none;
        }

        label {
            background: linear-gradient(102.61deg, #4b4b4b 25.57%, rgba(75, 75, 75, 0) 108.53%);
            background-size: 110%;
            border-radius: 9px;
            font-weight: 600;
            padding: 0.6em 1.2em;
            cursor: pointer;
            transition: 200ms;
            user-select: none;

            &:active {
                background-position: 95% 50%;
            }
        }

        .note {
            display: block;
            margin-top: 1em;
            color: #606060;
            font-weight: 600;
        }

        #thumbnail-preview {
            max-width: min(100%, 350px);
            border-radius: 20px;
        }
    }
</style>
