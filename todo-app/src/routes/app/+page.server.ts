import { elementsData } from '$lib/data.ts';

export async function load({ parent }) {
    const { user } = await parent();
    return {
        elementsData: user ? elementsData.filter(e => e.accountId === user.id) : elementsData,
    }
}