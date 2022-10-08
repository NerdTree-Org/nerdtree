<script lang="ts">
    export let max_page: number;
    export let current_page: number;
    export let callback: (page: number) => void;

    function range(start: number, end: number): Array<number> {
        let result = [];

        for (let i = start; i <= end; i++) {
            result.push(i);
        }

        return result;
    }

    function calculatePageRange(start: number, end: number): Array<number> {
        if (start == 1) {
            start += 1;
        }

        if (end >= max_page) {
            end -= end - max_page;
            end -= 1;
        }

        return range(start, end);
    }

    function updateCurrentPage(n: number) {
        current_page = n;
        callback(n);
    }
</script>

<div>
    <button on:click={() => current_page > 1 && updateCurrentPage(current_page - 1)}
        >&lt; Prev</button
    >
    <button class={current_page === 1 && 'current-page'} on:click={() => updateCurrentPage(1)}
        >1</button
    >
    {#if current_page !== max_page}
        {#if max_page - current_page > 1}
            <span>...</span>
        {/if}
        {#each calculatePageRange(current_page, current_page + 5) as n}
            <button
                class={n === current_page && 'current-page'}
                on:click={() => updateCurrentPage(n)}>{n}</button
            >
        {/each}
        {#if max_page - current_page > 1}
            <span>...</span>
        {/if}
    {/if}
    {#if 1 !== max_page}
        <button
            class={current_page === max_page && 'current-page'}
            on:click={() => updateCurrentPage(max_page)}>{max_page}</button
        >
    {/if}
    <button on:click={() => current_page < max_page && updateCurrentPage(current_page + 1)}
        >Next &gt;</button
    >
</div>

<style lang="scss">
    div {
        font-weight: 600;

        button {
            padding: 0.5em 1em;
            border-radius: 5px;

            &:hover {
                background: #303030;
            }
        }
    }
    .current-page {
        background: #404040;
    }
</style>
