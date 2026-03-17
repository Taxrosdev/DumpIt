<script>
	let { class: className, children } = $props();
</script>

<div
	class={'flex h-full w-full flex-col justify-center bg-background-card lg:h-120 lg:w-160 lg:flex-row lg:rounded-4xl ' +
		className}
>
	{@render children()}
</div>
