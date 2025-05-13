import { usersData } from '$lib/data.ts';

export function load({ cookies }) {
    let userIdCookie = cookies.get('userId');
    if (!userIdCookie) {
        return {
            user: null,
        }
    }

    let userId = parseInt(userIdCookie);
    let user = usersData.find((user) => user.id === userId)
    return {
        user: user ?? null,
    }
}