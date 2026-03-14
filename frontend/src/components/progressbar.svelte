<script lang="ts">
    const {value, max} = $props();
    
    const percentage = value/max * 100;
</script>

<div class="rounded-full bg-uploadcard-progressbar-background w-full h-2">
    <div class="rounded-full bg-uploadcard-progressbar-main h-2" style="width: {percentage}%;"></div>
</div>
