import { API } from '../../../api_wrapper';
import { page } from '$app/stores';
import xss from 'xss';
import MD from 'markdown-it';

/** @type {import('./$types').PageLoad} */

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export async function load({ params }) {
    let post;
    let post_author;

    const result = await API.post.query.id({
        post_id: params.id,
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
        }
    };

    return {
        post,
        post_author,
    }
}