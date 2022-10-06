<script lang="ts">
    import { AuthenticationStatus } from '../stores/user.ts';

    let user;
    AuthenticationStatus.subscribe((status) => {
        if (status.info) {
            user = status.info.user;
        }
    });
</script>

<div class="account-links flex justify-end">
    {#if $user}
        <div>
            <a href={`/profile/${$user.username}`}>
                <span class="link">
                    {$user.firstname + ' ' + $user.lastname}
                </span>
            </a>
        </div>
    {:else}
        <div class="flex gap-3">
            <a href="/login">
                <span class="link">Log In</span>
            </a>
            <span class="font-extralight p-2">or</span>
            <a href="/register">
                <span class="link" id="register"> Register </span>
            </a>
        </div>
    {/if}
</div>

<style lang="scss">
    .account-links {
        span {
            padding: 0.2em 0.5em 0.2em 0.5em;
        }

        .link {
            font-size: 1em;
            font-weight: 600;
            letter-spacing: -0.5px;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            user-select: none;
        }

        #register {
            background: linear-gradient(95.48deg, #28292c 0%, #131415 100%);
            border: 1px solid rgba(255, 255, 255, 0.12);
            border-radius: 5px;
        }
    }
</style>
