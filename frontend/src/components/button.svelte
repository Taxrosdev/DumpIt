<script lang="ts">
    let {onclick, class: className} = $props();
</script>

<button class={"rounded text-sm p-3 bg-button text-button-text " + className} {onclick}>
    <slot />
</button>
