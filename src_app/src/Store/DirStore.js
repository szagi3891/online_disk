//@flow

import { action, computed, observable } from "mobx";
import { OrderedMap } from 'immutable';
import type { CurrentHead, NodeItemType } from './Type';
import { HeadStore } from './HeadStore';
import { root } from "glamor";

const getDir = (hash: string, path: string): Promise<OrderedMap<string, NodeItemType>> => {
    return fetch(`/api/dir/${hash}${path}`)
        .then(response => response.json())
        .then(response => OrderedMap(response.files));
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

class BlobStoreItem {
    @observable _value: OrderedMap<string, NodeItemType> | null;

    constructor(hash: string, path: string) {
        this._value = null;

        console.info('TODO - inicjuję pobranie katalogu:', hash);
        getDir(hash, path).then(response => {
            console.info('Przeczytano dir z serwera', response);
            this._value = response;
        });
    }

    get value(): OrderedMap<string, NodeItemType> | null {
        return this._value;
    }
}

class BlobStore {
    +_data: Map<string, BlobStoreItem>;

    constructor() {
        this._data = new Map();
    }

    _getOrCreate(node_hash: string, node_path: string): BlobStoreItem {
        const item = this._data.get(node_hash);
        if (item) {
            return item;
        }
        const newItem = new BlobStoreItem(node_hash, node_path);
        this._data.set(node_hash, newItem);
        return newItem;
    }

    getDir(node_hash: string, node_path: string): OrderedMap<string, NodeItemType> | null {
        return this._getOrCreate(node_hash, node_path).value;
    }
}



export class DirItem {
    +_blob: BlobStore
    +_hash: string;
    +_path: string;

    constructor(blob: BlobStore, hash: string, path: string) {
        this._blob = blob;
        this._hash = hash;
        this._path = path;
    }

    @computed get value(): OrderedMap<string, NodeItemType> | null {
        return this._blob.getDir(this._hash, this._path);
    }

    @action add(dir: string): Promise<void> {

        console.info('ADD DIR', dir);

        return Promise.resolve();

        //TODO
        /*
        return addDir(dir).then((response: CurrentHead) => {
            this._setNewHead(response);
        });
        */
    }

    child(name: string): DirItem | null {
        return null;
    }
}

export class DirStore {
    +_head: HeadStore;
    +_blob: BlobStore;
    +_data: Map<string, DirItem>;

    constructor(head: HeadStore) {
        this._head = head;
        this._blob = new BlobStore();
        this._data = new Map();
    }

    getOrCreate(node_hash: string, node_path: string): DirItem {
        const item = this._data.get(node_hash);
        if (item) {
            return item;
        }
        const newItem = new DirItem(this._blob, node_hash, node_path);
        this._data.set(node_hash, newItem);
        return newItem;
    }

    @computed get root(): DirItem | null {
        const head = this._head.head;

        if (head === null) {
            return null;
        }

        return this.getOrCreate(head, "");
    }

    getItem(hash: string): DirItem | null {
        const item = this._data.get(hash);
        return item ? item : null;
    }
}
