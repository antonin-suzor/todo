import type { TODOElementData } from '$lib/types.ts';
import { elementsData } from '$lib/data.ts';

export interface appPageServerProps {
    elementsData: TODOElementData[],
}

export function load() : appPageServerProps {
    return {
        elementsData: elementsData,
    }
}