//@flow

import { action, observable } from "mobx";

import { getHead } from '../Api';
//import { Log } from './Log';

export class Store {
    @observable head: string | null;

    constructor() {
        this.head = null;
    }

    @action getHead() {
        console.info('Strzelam akcją po head');
        getHead().then((head: string) => {
            console.info('Otrzymany head', head);
        }).catch((error: mixed) => {
            console.info('Otrzymano błąd', error);
        })
    }

    @action addDir(dir: string) {
        const param = {
            dir
        };
        return fetch('/api/add_dir', {
            method: 'POST',
            body: JSON.stringify(param)
        });
    }
}
