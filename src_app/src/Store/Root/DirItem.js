//@flow
import { action, computed } from 'mobx';
import { List as IList, OrderedMap } from 'immutable';
import { BlobStore } from '../Blob/BlobStore';
import type { CurrentHead, NodeItemType } from '../Type';

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


export class DirItem {
    +_blob: BlobStore
    +_hash: string;
    +_path: IList<string>;

    constructor(blob: BlobStore, hash: string, path: IList<string>) {
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
        const value = this.value;

        if (value !== null) {
            const hashChild = value.get(name);
            if (hashChild) {
                return new DirItem(this._blob, hashChild.hash, this._path.push(name));
            }
        }

        return null;
    }
}
