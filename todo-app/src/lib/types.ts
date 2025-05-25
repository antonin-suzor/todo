export interface TODOElementData {
    id: number,
    title: string,
    done: boolean,
    description: string,
    accountId: number | null,
}

export interface UserData {
    id: number,
    name: string,
}

export interface TODOElementDataRequest {
    title: string,
    done: boolean,
    description: string,
}
