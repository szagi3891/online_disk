//@flow
import { action, computed, observable } from "mobx";
import { List as IList, OrderedMap } from 'immutable';
import type { NodeItemType } from './Type';
import { DirStore, DirItem } from './DirStore';
import { HeadStore } from './HeadStore';

export class PathStore {
    _head: HeadStore;
    _dir: DirStore;
    @observable _path: IList<string>;

    constructor(headStore: HeadStore, dirStore: DirStore) {
        this._head = headStore;
        this._dir = dirStore;
        this._path = IList();
    }

    get value(): IList<string> {
        return this._path;
    }

    /*
    @computed get serialized(): string {
        if (this._path.size === 0) {
            return '';
        }

        return `/${this._path.join('/')}`;
    }
    */

    @computed get currentHash(): string | null {
        return null;
    }

    /*
    //Sprawdza czy ta ścieżka jest poprawna
    _verifyPath(newPath: IList<string>): bool {

    }
    */

    /*
    getHashFromPath(path: IList<string>): string | null {
    }
    */

    @action goTo(newPath: IList<string>) {
        this._path = newPath;
    }

    _getItemByPath(parent: DirItem, path: IList<string>): DirItem | null {
        //TODO
        return null;
    }

    //TODO - na podstawie sciezki wybrac aktualnego noda
    @computed get currentItem(): DirItem | null {
        const rootItem = this._dir.root;
        if (rootItem) {
            return this._getItemByPath(rootItem, this._path);
        }

        return null;
    }

}

/*

a/b/c
    wez hash dla a/b
    wez katalog dla 
*/
