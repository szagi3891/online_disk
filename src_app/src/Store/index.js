//@flow

import { action, observable } from "mobx";
import { Map as IMap } from 'immutable';
import { DirStore } from './DirStore';
import type { CurrentHead } from './Type';
import { HeadStore } from './HeadStore';

export class Store {
    +head: HeadStore;
    +dir: DirStore;

    constructor() {
        this.head = new HeadStore();
        this.dir = new DirStore(this.head);
    }
}
