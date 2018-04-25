//@flow

import { action, observable } from "mobx";
import { OrderedMap } from 'immutable';
import type { CurrentHead, NodeItemType } from './Type';
import { HeadStore } from './HeadStore';

const getDir = (hash: string): Promise<OrderedMap<string, NodeItemType>> => {
    return fetch(`/api/node/${hash}/dir`)
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

    constructor(hash: string) {
        this._value = null;

        console.info('TODO - inicjuję pobranie katalogu:', hash);
        getDir(hash).then(response => {
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
    +_data: Map<string, DirStoreItem>;

    constructor(headStore: HeadStore) {
        this._headStore = headStore;
        this._data = new Map();
    }

    _getOrCreate(hash: string): DirStoreItem {
        const item = this._data.get(hash);
        if (item) {
            return item;
        }
        const newItem = new DirStoreItem(hash);
        this._data.set(hash, newItem);
        return newItem;
    }

    getDir(hash: string): OrderedMap<string, NodeItemType> | null {
        return this._getOrCreate(hash).value;
    }


    @action add(dir: string): Promise<void> {
        return addDir(dir).then((response: CurrentHead) => {
            this._headStore.saveHead(response);
        });
    }
}
