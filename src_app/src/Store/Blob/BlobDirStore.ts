import { observable } from 'mobx';
import { List as IList, OrderedMap } from 'immutable';
import { NodeItemType } from 'Store/Type';

const getDir = (hash: string, path: IList<string>): Promise<OrderedMap<string, NodeItemType>> => {
    const param = {
        node_hash: hash,
        path: path.toArray()
    };

    const fetchParam = {
        method: 'POST',
        body: JSON.stringify(param)
    };

    //@ts-ignore -- TODO
    return fetch('/api/dir/list', fetchParam)
        .then(response => response.json())
        .then(response => OrderedMap(response.files));
};

class BlobDirItem {
    @observable _value: OrderedMap<string, NodeItemType> | null;

    constructor(hash: string, path: IList<string>) {
        this._value = null;

        getDir(hash, path).then(response => {
            this._value = response;
        }).catch((error: any) => {
            console.error('Otrzymano błąd', error);
        });
    }

    get value(): OrderedMap<string, NodeItemType> | null {
        return this._value;
    }
}

export class BlobDirStore {
    readonly _data: Map<string, BlobDirItem>;

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

