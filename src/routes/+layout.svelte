<script>
	import '../app.css';
	import { BottomNav, BottomNavItem } from 'flowbite-svelte';
	import { ArchiveSolid, ClipboardListSolid, CogSolid, HomeSolid } from 'flowbite-svelte-icons';
	import { page } from '$app/stores';

	$: activeUrl = $page.url.pathname;
</script>

<!-- <Styles /> -->
<div class="flex flex-col min-h-screen" style="min-height: 100vh">
	<main>
		<slot />
	</main>
	<BottomNav {activeUrl} position="sticky" classInner="grid-cols-4" id="navbar" outerClass="mt-auto">
		<BottomNavItem btnName="Home" href="/">
			<HomeSolid/>
		</BottomNavItem>
		<BottomNavItem btnName="Lists" href="/lists">
			<ClipboardListSolid />
		</BottomNavItem>
		<BottomNavItem btnName="Items" href="/items">
			<ArchiveSolid />
		</BottomNavItem>
		<BottomNavItem btnName="Settings" href="/settings">
			<CogSolid />
		</BottomNavItem>
	</BottomNav>
</div>

<style>
	
</style>
