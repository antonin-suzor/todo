import { json } from '@sveltejs/kit';

import type { TODOElementDataRequest } from '$lib/types.ts';
import { elementsDataDelete, elementsDataModify } from '$lib/data.ts';

export async function PATCH({ cookies, params, request }) {
    let todoId = parseInt(params.id);
    if (Number.isNaN(todoId) || todoId <= 0) {
        return json({ message: 'id should be a positive number' }, { status: 400 })
    }

    let userIdCookie = cookies.get('userId') ?? 'NaN';
    let userId: number | undefined = parseInt(userIdCookie);
    if (Number.isNaN(userId)) {
        userId = undefined;
    }

    let modTodo: TODOElementDataRequest = await request.json();
    let modified = await elementsDataModify(todoId, modTodo, userId);
    if (!modified) {
        return new Response(null, { status: 403 });
    }
    return json(modified);
}

export async function DELETE({ cookies, params }) {
    let todoId = parseInt(params.id);
    if (Number.isNaN(todoId) || todoId <= 0) {
        return json({ message: 'id should be a positive number' }, { status: 400 })
    }

    let userIdCookie = cookies.get('userId') ?? 'NaN';
    let userId: number | undefined = parseInt(userIdCookie);
    if (Number.isNaN(userId)) {
        userId = undefined;
    }

    let deletionResult = await elementsDataDelete(todoId, userId);
    return new Response(null, { status: deletionResult ? 200 : 403 });
}