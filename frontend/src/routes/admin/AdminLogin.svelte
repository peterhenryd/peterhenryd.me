<script lang="ts">
    import {api_endpoint} from "$lib/api";
    import {page} from "$app/stores";

    $:host = $page.url.host;
</script>

<div class="grid h-screen bg-blue-900">
    <form method="post" action={api_endpoint + `/admin?redirect=${encodeURIComponent('http://' + host + '/admin')}`} class="bg-white rounded-3xl admin-login hover:shadow-2xl transition-shadow shadow-xl p-7 flex flex-col gap-4 place-self-center -translate-y-16">
        <h1 class="text-4xl">Login</h1>
        <input placeholder="Password" class="max-w-full placeholder-gray-500 transition-colors hover:bg-gray-200 bg-gray-300 p-2 rounded-lg" name="admin_key" type="password" />
        <button class="bg-blue-600 text-white hover:bg-blue-500 transition-colors uppercase p-2 rounded-lg tracking-widest" type="submit">Submit</button>
    </form>
</div>
<style>
    * {
        font-family: azo-sans-web, serif;
    }

    .admin-login {
        width: 400px;
        height: min-content;
    }
</style>