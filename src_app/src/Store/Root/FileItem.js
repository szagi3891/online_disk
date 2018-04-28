//@flow

import { action, computed } from 'mobx';
import { List as IList, OrderedMap } from 'immutable';
import { BlobStore } from '../Blob/BlobStore';
import { HeadStore } from '../HeadStore';

export class FileItem {
    +_head: HeadStore;
    +_blob: BlobStore
    +_hash: string;
    +_path: IList<string>;

    constructor(head: HeadStore, blob: BlobStore, hash: string, path: IList<string>) {
        this._head = head;
        this._blob = blob;
        this._hash = hash;
        this._path = path;
    }
}
