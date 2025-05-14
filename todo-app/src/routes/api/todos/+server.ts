import { json } from '@sveltejs/kit';

import type { TODOElementData } from '$lib/types.ts';
import { elementsDataNew } from '$lib/data.ts';

export async function POST({ cookies, request }) {
    let userIdCookie = cookies.get('userId') ?? '0';
    let userId = parseInt(userIdCookie);
    if (Number.isNaN(userId)) {
        userId = 0;
    }

    let newTodo: TODOElementData = await request.json();
    if (newTodo.accountId != userId) {
        return json({ message: 'user cookie and element account do not match' }, { status: 400 })
    }

    return json(await elementsDataNew(newTodo));
}