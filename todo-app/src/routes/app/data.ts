import type { TODOElementData } from '$lib/types.ts';

export const elementsData: TODOElementData[] = [
    {
        id: 1,
        title: 'TODO element',
        done: false,
        description: 'Element description',
    },
    {
        id: 2,
        title: 'DONE element',
        done: true,
        description: 'Element description',
    },
];