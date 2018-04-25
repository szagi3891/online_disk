//@flow
import { action, observable } from "mobx";
import type { CurrentHead } from './Type';

const getHead = (): Promise<CurrentHead> => {
    return fetch('/api/head')
        .then(response => response.json());
};

export class HeadStore {
    @observable _head: CurrentHead | null;

    constructor() {
        this._head = null;


        console.info('Strzelam akcją po head');
        getHead().then((head: CurrentHead) => {
            console.info('Otrzymany head', head);
            this.saveHead(head);
        }).catch((error: mixed) => {
            console.info('Otrzymano błąd', error);
        })
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
