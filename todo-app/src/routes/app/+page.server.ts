import { elementsData } from '$lib/data.ts';

export async function load({ parent }) {
    const { user } = await parent();
    return {
        elementsData: await elementsData(user?.id),
    }
}