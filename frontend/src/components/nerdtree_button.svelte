<script lang="ts" context="module">
    export enum ButtonType {
        Hazy = 'hazy-button',
        Smooth = 'smooth-button'
    }
</script>

<script lang="ts">
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    export let on_click: () => void = () => {};
    export let type: ButtonType;
</script>

<button class={`button ${type}`} on:click={on_click}>
    <slot />
</button>

<style lang="scss">
    .button {
        display: inline;
        letter-spacing: -0.5px;
        font-weight: 600;
        font-size: 17px;
        user-select: none;
        cursor: pointer;
    }

    .hazy-button {
        background: linear-gradient(100.35deg, #535353 29%, rgba(83, 83, 83, 0) 100.36%);
        background-size: 130%;
        border: 1px solid rgba(255, 255, 255, 0.24);
        border-radius: 5px;
        transition: 200ms;
        padding: 0.5em 1.5em;

        &:active {
            background-position-x: 100%;
            background-position-y: 100%;
            background-size: 200%;
        }
    }

    .smooth-button {
        background: linear-gradient(102.61deg, #4b4b4b 25.57%, rgba(75, 75, 75, 0) 108.53%);
        border-radius: 9px;
        color: white;
        padding: 0.5em 3em;
    }
</style>
