//@flow
import { computed } from 'mobx';
import { List as IList } from 'immutable';
import { HeadStore } from '../HeadStore';
import { BlobStore } from '../Blob/BlobStore';
import { DirItem } from './DirItem';
import { FileItem } from './FileItem';
import { PathStore } from '../PathStore';

const findItemDir = (parent: DirItem, currentPath: IList<string>): [IList<DirItem>, null | FileItem] => {
    const first = currentPath.first();

    if (typeof first === 'string') {
        const nextChild = parent.child(first);

        if (nextChild === null) {
            return [IList(), null];
        }

        if (nextChild instanceof FileItem) {
            return [IList(), nextChild];
        }
        
        if (nextChild instanceof DirItem) {
            const [dirList, lastElement] = findItemDir(nextChild, currentPath.shift());
            return [dirList.unshift(nextChild), lastElement];
        }

        throw Error('Store.Root.findItemDir: Nieobsłużone odgałęzienie');
    }

    return [IList(), null];
};

type CurrentPathNodesType = {|
    path: IList<DirItem>,
    last: null | FileItem
|};

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

        return new DirItem(null, this._head, this._blob, head, IList(), 'ROOT');
    }

    @computed get currentPathNodes(): CurrentPathNodesType {
        const root = this.root;
        if (!root) {
            return {
                path: IList(),
                last: null
            };
        }

        const [path, last] = findItemDir(root, this._path.value);
        return {
            path: path.unshift(root),
            last
        };
    }
}
