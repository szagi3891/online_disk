//@flow

import { action, observable } from "mobx";
import { Map as IMap, OrderedMap } from 'immutable';
import { RootStore } from './Root/RootStore';
import type { CurrentHead, NodeItemType } from './Type';
import { HeadStore } from './HeadStore';
import { PathStore } from './PathStore';

export class Store {
    +head: HeadStore;
    +path: PathStore;
    +root: RootStore;

    constructor() {
        this.head = new HeadStore();
        this.path = new PathStore();
        this.root = new RootStore(this.head, this.path);
    }
}
