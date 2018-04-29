//@flow
import { action, computed } from 'mobx';
import { List as IList, OrderedMap } from 'immutable';
import { BlobStore } from '../Blob/BlobStore';
import type { CurrentHead, NodeItemType } from '../Type';
import { FileItem } from './FileItem';
import { HeadStore } from '../HeadStore';

const addDir = (node_hash: string, path: IList<string>, dir: string): Promise<CurrentHead> => {
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
    +_head: HeadStore;
    +_blob: BlobStore;
    +_hash: string;
    +_path: IList<string>;

    constructor(head: HeadStore, blob: BlobStore, hash: string, path: IList<string>) {
        this._blob = blob;
        this._hash = hash;
        this._path = path;
    }

    @computed get value(): OrderedMap<string, NodeItemType> | null {
        return this._blob.getDir(this._hash, this._path);
    }

    @action add(dir: string): Promise<void> {
        return addDir(this._hash, this._path, dir).then((response: CurrentHead) => {
            this._head.saveHead(response);
        });
    }

    child(name: string): DirItem | FileItem | null {
        const value = this.value;

        if (value !== null) {
            const hashChild = value.get(name);
            if (hashChild) {
                if (hashChild.is_dir) {
                    return new DirItem(
                        this._head,
                        this._blob,
                        hashChild.hash,
                        this._path.push(name)
                    );
                } else {
                    return new FileItem(
                        this._head,
                        this._blob,
                        hashChild.hash,
                        this._path.push(name),
                        name
                    );
                }
            }
        }

        return null;
    }
}
