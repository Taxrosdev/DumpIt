<script>
	let { class: className, children } = $props();
</script>

<div
	class={'flex flex-col justify-center bg-background-card lg:h-120 lg:w-90 lg:rounded-4xl ' +
		className}
>
	{@render children()}
</div>
