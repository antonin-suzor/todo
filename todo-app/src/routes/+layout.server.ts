import { usersData } from '$lib/data.ts';

export async function load({ cookies }) {
    let userIdCookie = cookies.get('userId');
    if (!userIdCookie) {
        return {
            user: null,
        }
    }

    let userId = parseInt(userIdCookie);
    let users = await usersData(userId);
    return {
        user: users.length > 0 ? users[0] : null,
    }
}