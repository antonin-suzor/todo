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

export function usersData() {
    return sql<UserData[]>`SELECT id, name FROM account;`;
}