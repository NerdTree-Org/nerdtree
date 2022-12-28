<script lang="ts">
    export let idx = 0;
    let selections = [false, false, false, false];
    selections[idx] = true;
</script>

<div class="sidebar pt-10 flex flex-wrap flex-col items-start justify-start h-inherit">
    <a class={selections[0] && 'selected'} href="/settings">Basic Information</a>
    <a class={selections[1] && 'selected'} href="/settings/discord">Discord token</a>
    <a class={selections[2] && 'selected'} href="/settings/security">Security Information</a>
    <a class={`red ${selections[3] && 'selected'}`} href="/settings/leave">Leave NerdTree</a>
</div>

<style lang="scss">
    .sidebar {
        min-width: 250px;
        background: #101010;

        a {
            padding: 1em 1.5em;
            font-weight: 600;
            width: 100%;
            text-align: left;

            &.selected {
                background: #0c0c0c;
            }

            &.red {
                color: #ff3232;
            }
        }
    }

    @media only screen and (max-width: 1010px) {
        .sidebar {
            width: 100%;
            height: max-content;
            padding-top: 0;
        }
    }
</style>
