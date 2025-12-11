import { PUBLIC_API_URL } from "$env/static/public";

export async function load({ fetch }) {
	const res = await fetch(`${PUBLIC_API_URL}/hello`);
	const message = await res.json();

	return { message };
}
