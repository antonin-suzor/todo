import { json } from '@sveltejs/kit';

import type { NewTodoRequest } from '$lib/types.ts';
import { elementsDataNew } from '$lib/data.ts';

export async function POST({ cookies, request }) {
    let userIdCookie = cookies.get('userId') ?? 'NaN';
    let userId: number | undefined = parseInt(userIdCookie);
    if (Number.isNaN(userId)) {
        userId = undefined;
    }

    let newTodo: NewTodoRequest = await request.json();
    return json(await elementsDataNew(newTodo, userId));
}