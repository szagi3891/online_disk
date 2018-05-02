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

    return fetch(`/api/add_dir/${node_hash}/${path.join('/')}`, fetchParam)
        .then(response => response.json());
};


export class DirItem {
    +_parent: DirItem | null;
    +_head: HeadStore;
    +_blob: BlobStore;
    +_hash: string;
    +_path: IList<string>;

    constructor(parent: DirItem | null, head: HeadStore, blob: BlobStore, hash: string, path: IList<string>) {
        this._parent = parent;
        this._head = head;
        this._blob = blob;
        this._hash = hash;
        this._path = path;
    }

    get path(): IList<string> {
        return this._path;
    }

    @computed get _listNodes(): OrderedMap<string, NodeItemType> | null {
        return this._blob.getDir(this._hash, this._path);
    }

    @action add(dir: string): Promise<void> {
        return addDir(this._hash, this._path, dir).then((response: CurrentHead) => {
            this._head.saveHead(response);
        });
    }

    @computed get childList(): OrderedMap<string, DirItem | FileItem> | null {
        const listNodes = this._listNodes;

        if (listNodes !== null) {
            return listNodes.map(
                (item: NodeItemType, name: string): DirItem | FileItem => this._mapNode(name, item)
            );
        }

        return null;
    }

    _mapNode(name: string, item: NodeItemType): DirItem | FileItem {
        if (item.is_dir) {
            return new DirItem(
                this,
                this._head,
                this._blob,
                item.hash,
                this._path.push(name)
            );
        }
        return new FileItem(
            this._head,
            this._blob,
            item.hash,
            this._path.push(name),
            name
        );
    }

    child(name: string): DirItem | FileItem | null {
        const listNodes = this._listNodes;

        if (listNodes !== null) {
            const hashChild = listNodes.get(name);
            if (hashChild) {
                return this._mapNode(name, hashChild);
            }
        }

        return null;
    }

    get parent(): DirItem | null {
        return this._parent;
    }
}
