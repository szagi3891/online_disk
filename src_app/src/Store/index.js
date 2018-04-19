//@flow

import { action } from "mobx";

import { getHead } from '../Api';
//import { Log } from './Log';

export class Store {
    //+log: Log;

    constructor() {
        //this.log = new Log();
    }

    @action getHead() {
        console.info('Strzelam akcją po head');
        getHead().then((head: string) => {
            console.info('Otrzymany head', head);
        }).catch((error: mixed) => {
            console.info('Otrzymano błąd', error);
        })
    }
}
