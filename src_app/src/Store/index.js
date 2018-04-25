//@flow

import { action, observable } from "mobx";
import { Map as IMap } from 'immutable';

type CurrentHead = {
    head: string,
    counter: number,
};

const getHead = (): Promise<CurrentHead> => {
    return fetch('/api/head')
        .then(response => response.json());
};

const addDir = (dir: string): Promise<CurrentHead> => {
    const param = {
        dir
    };
    const fetchParam = {
        method: 'POST',
        body: JSON.stringify(param)
    };

    return fetch('/api/add_dir', fetchParam)
        .then(response => response.json());
};

type NodeItemType = {|
    +is_dir: bool,
    +hash: string,
|};

const getDir = (hash: string): Promise<IMap<string, NodeItemType>> => {
    return fetch(`/api/node/${hash}/dir`)
        .then(response => response.json())
        .then(response => IMap(response));
};

export class Store {
    @observable head: CurrentHead | null;

    constructor() {
        this.head = null;
    }

    @action saveHead(head: CurrentHead) {
        if (this.head === null || this.head.counter < head.counter) {
            console.info('Zapisuję nowy head', )
            this.head = head;
        }
    }

    @action getHead() {
        console.info('Strzelam akcją po head');
        getHead().then((head: CurrentHead) => {
            console.info('Otrzymany head', head);
            this.saveHead(head);
        }).catch((error: mixed) => {
            console.info('Otrzymano błąd', error);
        })
    }

    @action addDir(dir: string): Promise<void> {
        return addDir(dir).then((response: CurrentHead) => {
            this.saveHead(response);
        });
    }

    @action getDir(hash: string) {
        console.info('TODO - inicjuję pobranie katalogu:', hash);
        getDir(hash).then(response => {
            console.info('Przeczytano dir z serwera', response);
        });
    }
}
