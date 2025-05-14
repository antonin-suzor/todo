import type { TODOElementData, UserData } from '$lib/types.ts';
import sql from '$lib/db.ts';

export function elementsData(userId?: number) {
    return sql<TODOElementData[]>`
        SELECT id, title, done, description, account_id as "accountId"
        FROM todo_element_data ${
        userId 
                ? sql`WHERE account_id = ${userId};`
                : sql``
        }
    ;`;
}

export async function elementsDataNew(newTodo: TODOElementData) {
    let res: {id: number}[] = await sql`
        INSERT INTO todo_element_data
            ${sql({ ...newTodo, account_id: newTodo.accountId },
                    'title', 'done', 'description', 'account_id')
            }
            RETURNING id
        ;`;
    let id = res[0].id
    return { ...newTodo, id }
}

export function usersData() {
    return sql<UserData[]>`SELECT id, name FROM account;`;
}