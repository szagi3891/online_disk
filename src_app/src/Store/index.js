//@flow

import { action, observable } from "mobx";
import { Map as IMap } from 'immutable';
import { DirStore } from './DirStore';
import type { CurrentHead } from './Type';

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

class HeadStore {
    @observable _head: CurrentHead | null;

    constructor() {
        this._head = null;
    }

    @action saveHead(newHead: CurrentHead) {
        if (this._head === null || this._head.counter < newHead.counter) {
            console.info('Zapisuję nowy head', newHead);
            this._head = newHead;
        }
    }

    get head(): string | null {
        return this._head ? this._head.head : null;
    }
    get counter(): number | null {
        return this._head ? this._head.counter : null;
    }
}

export class Store {
    +head: HeadStore;
    +dir: DirStore;

    constructor() {
        this.head = new HeadStore();
        this.dir = new DirStore();
    }

    @action getHead() {
        console.info('Strzelam akcją po head');
        getHead().then((head: CurrentHead) => {
            console.info('Otrzymany head', head);
            this.head.saveHead(head);
        }).catch((error: mixed) => {
            console.info('Otrzymano błąd', error);
        })
    }

    @action addDir(dir: string): Promise<void> {
        return addDir(dir).then((response: CurrentHead) => {
            this.head.saveHead(response);
        });
    }
}
