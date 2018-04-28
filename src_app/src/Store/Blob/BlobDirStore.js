//@flow
import { observable } from 'mobx';
import { List as IList, OrderedMap } from 'immutable';
import type { NodeItemType } from '../Type';

const getDir = (hash: string, path: IList<string>): Promise<OrderedMap<string, NodeItemType>> => {
    return fetch(`/api/dir/${hash}/${path.join('/')}`)
        .then(response => response.json())
        .then(response => OrderedMap(response.files));
};

class BlobDirItem {
    @observable _value: OrderedMap<string, NodeItemType> | null;

    constructor(hash: string, path: IList<string>) {
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

export class BlobDirStore {
    +_data: Map<string, BlobDirItem>;

    constructor() {
        this._data = new Map();
    }

    getOrCreate(node_hash: string, node_path: IList<string>): BlobDirItem {
        const item = this._data.get(node_hash);
        if (item) {
            return item;
        }
        const newItem = new BlobDirItem(node_hash, node_path);
        this._data.set(node_hash, newItem);
        return newItem;
    }

    getDir(node_hash: string, node_path: IList<string>): OrderedMap<string, NodeItemType> | null {
        return this.getOrCreate(node_hash, node_path).value;
    }
}

