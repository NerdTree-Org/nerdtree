/** @type {import('./$types').PageLoad} */
import { API } from '../../../api_wrapper';
import { get } from 'svelte/store';
import { AuthenticationStatus } from '../../../stores/user';

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export async function load({ params }) {
    const username = params.username;
    let user;
    let posts;

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

    return {
        user,
        posts
    }
}