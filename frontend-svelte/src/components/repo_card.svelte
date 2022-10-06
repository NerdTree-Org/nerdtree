<script lang="ts">
    import NerdTreeButton from '../components/nerdtree_button.svelte';
    import { ButtonType } from '../components/nerdtree_button.svelte';

    export let repo: string;
    export let maintainers: string;
    export let link: string;
    export let issues: string;
</script>

<div class="repo-card">
    <!--
        TODO: find a better way to get social media preview
    -->
    <img
        src={'https://opengraph.githubassets.com/e99ccd45d06538a422ca2012bb9e73f14e9a07ddeb56cc70f673f4fd8129e849/' +
            repo}
        alt="repo-preview"
    />
    <div>
        <h3>{repo}</h3>
        <h4>Maintainers: {maintainers.join(', ')}</h4>
        <div class={'flex gap-3.5'}>
            <a href={link}>
                <NerdTreeButton type={ButtonType.Hazy}>Repo</NerdTreeButton>
            </a>
            <a href={issues}>
                <NerdTreeButton type={ButtonType.Hazy}>Issues</NerdTreeButton>
            </a>
        </div>
    </div>
</div>

<style lang="scss">
    .repo-card {
        background: linear-gradient(122.11deg, #0c0c0c -3.63%, rgba(117, 117, 117, 0.22) 160.76%);
        border: 1px solid rgba(255, 255, 255, 0.24);
        border-radius: 12px;
        padding: 1em;
        max-height: 300px;
        display: flex;
        gap: 30px;

        img {
            border-radius: 12px;
            max-width: 400px;
        }

        div {
            margin-right: 2em;

            h3,
            h4 {
                display: block;
            }
            a {
                margin-top: 1em;
            }
        }
    }
</style>
