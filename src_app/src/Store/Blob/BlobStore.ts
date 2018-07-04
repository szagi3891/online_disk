import { List as IList, OrderedMap } from 'immutable';
import { BlobDirStore } from 'Store/Blob/BlobDirStore';
import { NodeItemType } from 'Store/Type';

export class BlobStore {
    readonly _dirStore: BlobDirStore;

    constructor() {
        this._dirStore = new BlobDirStore();
    }

    getDir(node_hash: string, node_path: IList<string>): OrderedMap<string, NodeItemType> | null {
        return this._dirStore.getDir(node_hash, node_path);
    }
}
