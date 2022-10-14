<script lang="ts">
    import type { User } from '../interfaces/user';
    import { API } from '../api_wrapper';
    import PlaceHolderProfilePicture from '../images/default-avatar.png';
    import { ENV } from '../env.js';

    export let body: string;
    export let creation_date: Date;
    export let author_id: string;

    let author: User | null;

    (async () => {
        const result = await API.user.query.id({
            id: author_id
        });

        if (result.success) {
            author = result.value;
        }
    })();
</script>

<div class="comment flex gap-5 p-5">
    <div>
        <img
            alt={`${author?.username ? author.username : 'deleted_user'}'s profile picture`}
            src={`${
                author?.profile_pic
                    ? `${ENV.api_address}/static/${author.profile_pic}`
                    : PlaceHolderProfilePicture
            }`}
        />
    </div>
    <div>
        <a href={`/u/${author?.username ? author.username : 'deleted_user'}`}>
            u/{author?.username ? author.username : 'deleted_user'}
        </a>
        <p>{creation_date.toDateString()}</p>
        <span>{body}</span>
    </div>
</div>

<style lang="scss">
    .comment {
        background: #232323;
        border-radius: 15px;

        a,
        p {
            color: #696969;
        }

        span {
            font-size: 20px;
        }
    }

    img {
        width: 40px;
        border-radius: 50%;
    }
</style>
