import {api_endpoint, type Fetch} from "$lib/api/index";

export async function isAdmin(a: Fetch) {
    const res = await fetch(api_endpoint + "/admin", {
        headers: [
            ["Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS"],
            ["Access-Control-Allow-Origin", "*"],
            ["Access-Control-Allow-Headers", "*"],
        ]
    });
    console.log(res);
}