import {isAdmin} from "$lib/api/admin";

/** @type {import('../../../../.svelte-kit/types/src/routes').PageLoad} */
export async function load({ fetch }: any) {
    return {
        isAdmin: await isAdmin(fetch)
    };
}