<script>
	import '../styles/app.scss';
	import { writable } from 'svelte/store';
	import { setContext } from 'svelte';
	import Footer from '../components/footer.svelte';

	const layout = writable('default_layout');
	setContext('setLayout', layout.set);
</script>

{#if $layout === 'default_layout'}
	<Footer />
{/if}
