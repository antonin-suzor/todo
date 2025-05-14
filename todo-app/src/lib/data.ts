import type { NewTodoRequest, TODOElementData, UserData } from '$lib/types.ts';
import sql from '$lib/db.ts';

export async function elementsData(userId?: number): Promise<TODOElementData[]> {
    return sql<TODOElementData[]>`
        SELECT id, title, done, description, account_id as "accountId"
        FROM todo_element_data ${
        userId 
                ? sql`WHERE account_id = ${userId}`
                : sql``
        }`
}

export async function elementsDataNew(newTodo: NewTodoRequest, userId?: number): Promise<TODOElementData> {
    let res: {id: number}[] = await sql`
        INSERT INTO todo_element_data ${
        sql({ ...newTodo, account_id: userId ?? null },
                'title', 'done', 'description', 'account_id')
        } RETURNING id`
    let id = res[0].id;
    return { ...newTodo, id, accountId: userId ?? null }
}

export async function usersData(userId?: number): Promise<UserData[]> {
    return sql<UserData[]>`SELECT id, name FROM account ${userId ? sql`WHERE id = ${userId}` : sql``}`
}