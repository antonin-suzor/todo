import type { TODOElementDataRequest, TODOElementData, UserData } from '$lib/types.ts';
import sql from '$lib/db.ts';

export async function elementsData(userId?: number): Promise<TODOElementData[]> {
    return sql<TODOElementData[]>`
        SELECT id, title, done, description, account_id as "accountId"
        FROM todo_element_data ${userId ? sql`WHERE account_id = ${userId}` : sql`WHERE account_id IS NULL`}`
}

export async function elementsDataNew(newTodo: TODOElementDataRequest, userId?: number): Promise<TODOElementData> {
    let res: {id: number}[] = await sql`
        INSERT INTO todo_element_data ${
        sql({ ...newTodo, account_id: userId ?? null },
                'title', 'done', 'description', 'account_id')
        } RETURNING id`

    let id = res[0].id;
    return { ...newTodo, id, accountId: userId ?? null }
}

export async function elementsDataModify(todoId: number, modTodo: TODOElementDataRequest, userId?: number): Promise<TODOElementData | null> {
    let res: {id: number}[] = await sql`
        UPDATE todo_element_data SET ${
        sql({ ...modTodo, account_id: userId ?? null },
                'title', 'done', 'description', 'account_id')
        } WHERE id = ${todoId} ${userId ? sql`AND account_id = ${userId}` : sql`AND account_id IS NULL`}
        RETURNING id`

    if (res.length === 0) {
        return null;
    }
    let id = res[0].id;
    return { ...modTodo, id, accountId: userId ?? null }
}

export async function elementsDataDelete(todoId: number, userId?: number): Promise<boolean> {
    let res: {id: number}[] = await sql`
        DELETE FROM todo_element_data
        WHERE id = ${todoId} ${userId ? sql`AND account_id = ${userId}` : sql`AND account_id IS NULL`}
        RETURNING id`
    return res.length > 0;
}

export async function usersData(userId?: number): Promise<UserData[]> {
    return sql<UserData[]>`SELECT id, name FROM account ${userId ? sql`WHERE id = ${userId}` : sql``}`
}