//@flow

export const getHead = (): Promise<string> => {
    return fetch('/api/head')
        .then(response => response.json())
        .then(response => response.head);
};

