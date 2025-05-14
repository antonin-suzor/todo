import { usersData } from '$lib/data.ts';

export async function load({ cookies }) {
    let userIdCookie = cookies.get('userId');
    if (!userIdCookie) {
        return {
            user: null,
        }
    }

    let userId = parseInt(userIdCookie);
    let user = (await usersData()).find((user) => user.id === userId)
    return {
        user: user ?? null,
    }
}