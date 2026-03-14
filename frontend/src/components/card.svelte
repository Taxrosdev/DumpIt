<script>
let {class: className} = $props();
</script>

<div class={"flex flex-col justify-center lg:h-120 lg:w-90 lg:rounded-4xl bg-background-card " + className}>
    <slot />
</div>

