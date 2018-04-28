//@flow
import { action, computed, observable } from "mobx";
import { OrderedMap, List as IList } from 'immutable';
import type { CurrentHead, NodeItemType } from '../Type';
import { HeadStore } from '../HeadStore';
import { BlobStore } from '../Blob/BlobStore';
import { DirItem } from './DirItem';
import { PathStore } from '../PathStore';

export class RootStore {
    +_head: HeadStore;
    +_path: PathStore;
    +_blob: BlobStore;

    constructor(head: HeadStore, path: PathStore) {
        this._head = head;
        this._path = path;
        this._blob = new BlobStore();
    }

    @computed get root(): DirItem | null {
        const head = this._head.head;

        if (head === null) {
            return null;
        }

        return new DirItem(this._head, this._blob, head, IList());
    }

    /*
    @computed get currentPath(): [IList<DirItem>, null | FileItem] {

    }
    */
}


    /*
    @computed get serialized(): string {
        if (this._path.size === 0) {
            return '';
        }

        return `/${this._path.join('/')}`;
    }
    */

    /*
    @computed get currentHash(): string | null {
        return null;
    }
    */

    /*
    //Sprawdza czy ta ścieżka jest poprawna
    _verifyPath(newPath: IList<string>): bool {

    }
    */

    /*
    getHashFromPath(path: IList<string>): string | null {
    }
    */

    /*
    _getFirst(list: IList<string>): [string, IList<string>] | null {
        const first = list.first();
        if (typeof first === 'string') {
            return [first, list.shift()];
        }

        return null;
    }

    _getItemByPath(parent: DirItem, path: IList<string>): DirItem | null {
        const firstResult = this._getFirst(path);

        if (!firstResult) {
            return parent;
        }

        const [ first, rest ] = firstResult;

        const child = parent.child(first);
        if (!child) {
            return null;
        }

        return this._getItemByPath(child, rest);
    }

    //TODO - na podstawie sciezki wybrac aktualnego noda
    @computed get currentItem(): DirItem | null {
        const rootItem = this._dir.root;
        if (rootItem) {
            return this._getItemByPath(rootItem, this._path);
        }

        return null;
    }
    */

/*

a/b/c
    wez hash dla a/b
    wez katalog dla 
*/
