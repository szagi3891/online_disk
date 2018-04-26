//@flow

import { action, observable } from "mobx";
import { OrderedMap } from 'immutable';
import type { CurrentHead, NodeItemType } from './Type';
import { HeadStore } from './HeadStore';
import { PathStore } from './PathStore';

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

class DirStoreItem {
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

export class DirStore {
    +_headStore: HeadStore;
    +_pathStore: PathStore;
    +_data: Map<string, DirStoreItem>;

    constructor(headStore: HeadStore, pathStore: PathStore) {
        this._headStore = headStore;
        this._pathStore = pathStore;
        this._data = new Map();
    }

    _getOrCreate(node_hash: string, node_path: string): DirStoreItem {
        const item = this._data.get(node_hash);
        if (item) {
            return item;
        }
        const newItem = new DirStoreItem(node_hash, node_path);
        this._data.set(node_hash, newItem);
        return newItem;
    }

    getDir(node_hash: string, node_path: string): OrderedMap<string, NodeItemType> | null {
        return this._getOrCreate(node_hash, node_path).value;
    }

    @action add(dir: string): Promise<void> {
        return addDir(dir).then((response: CurrentHead) => {
            this._headStore.saveHead(response);
        });
    }

    //Ta metoda będzie używana przez add_dir i inne które operują na nodzie
    //Można by się pokusić żeby ta metoda zwracała od razu całą lokalizację /hash/path/do/noda
    getNodeHashFromCurrentPath(): string | null {
        //TODO
        return null;
    }
}
