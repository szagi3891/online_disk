import { RootStore } from './Root/RootStore';
import { HeadStore } from './HeadStore';
import { PathStore } from './PathStore';

export class Store {
    readonly head: HeadStore;
    readonly path: PathStore;
    readonly root: RootStore;

    constructor() {
        this.head = new HeadStore();
        this.path = new PathStore();
        this.root = new RootStore(this.head, this.path);
    }
}
