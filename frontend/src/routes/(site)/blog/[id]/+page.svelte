<script>
    export let data;

</script>
<div class="flex flex-col gap-3">
    <h1 class="text-4xl">{data.post.title}</h1>
    <p>{@html data.post.content}</p>
</div>