//@flow
import { action, computed, observable } from "mobx";
import { OrderedMap, List as IList } from 'immutable';
import type { CurrentHead, NodeItemType } from '../Type';
import { HeadStore } from '../HeadStore';
import { BlobStore } from '../Blob/BlobStore';
import { DirItem } from './DirItem';

export class RootStore {
    +_head: HeadStore;
    +_blob: BlobStore;

    constructor(head: HeadStore) {
        this._head = head;
        this._blob = new BlobStore();
    }

    @computed get root(): DirItem | null {
        const head = this._head.head;

        if (head === null) {
            return null;
        }

        return new DirItem(this._blob, head, IList());
    }

    /*
    @computed get currentPath(): [IList<DirItem>, null | FileItem] {

    }
    */
}
