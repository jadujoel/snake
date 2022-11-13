export function unwrap<T>(val: T | undefined | null): T{
    if (val === undefined || val === null) {
        throw new Error("Value is undefined or null");
    }
    return val;
}

export function getElementById<T extends HTMLElement>(id: string): T {
    return unwrap(document.getElementById(id)) as T;
}
