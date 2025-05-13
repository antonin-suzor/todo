import type { TODOElementData, UserData } from '$lib/types.ts';

export const elementsData: TODOElementData[] = [
    {
        id: 11,
        title: 'TODO element for user 1',
        done: false,
        description: 'Element description',
        accountId: 1,
    },
    {
        id: 12,
        title: 'DONE element for user ABC',
        done: true,
        description: 'Element description',
        accountId: 1,
    },
    {
        id: 21,
        title: 'TODO element for user 2',
        done: false,
        description: 'Element description',
        accountId: 2,
    },
    {
        id: 22,
        title: 'DONE element for user DEF',
        done: true,
        description: 'Element description',
        accountId: 2,
    },
];

export const usersData: UserData[] = [
    {
        id: 1,
        name: 'user ABC',
    },
    {
        id: 1,
        name: 'user DEF',
    },
];