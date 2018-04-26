//@flow

import { action, observable } from "mobx";
import { Map as IMap } from 'immutable';
import { DirStore } from './DirStore';
import type { CurrentHead } from './Type';
import { HeadStore } from './HeadStore';
import { PathStore } from './PathStore';

export class Store {
    +head: HeadStore;
    +path: PathStore;
    +dir: DirStore;

    constructor() {
        this.head = new HeadStore();
        this.path = new PathStore();
        this.dir = new DirStore(this.head, this.path);
    }
}
