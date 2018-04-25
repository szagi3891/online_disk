//@flow

import { action, observable } from "mobx";
import { Map as IMap } from 'immutable';
import type { NodeItemType } from './Type';

const getDir = (hash: string): Promise<IMap<string, NodeItemType>> => {
    return fetch(`/api/node/${hash}/dir`)
        .then(response => response.json())
        .then(response => IMap(response));
};

class DirStoreItem {
    @observable _value: IMap<string, NodeItemType> | null;

    constructor(hash: string) {
        this._value = null;

        console.info('TODO - inicjuję pobranie katalogu:', hash);
        getDir(hash).then(response => {
            console.info('Przeczytano dir z serwera', response);
            this._value = response;
        });
    }

    get value(): IMap<string, NodeItemType> | null {
        return this._value;
    }
}

export class DirStore {
    +_data: Map<string, DirStoreItem>;

    constructor() {
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

    getDir(hash: string): IMap<string, NodeItemType> | null {
        return this._getOrCreate(hash).value;
    }
}
