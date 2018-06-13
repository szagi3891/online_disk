import { action, observable } from 'mobx';
import { List as IList } from 'immutable';

export class PathStore {
    @observable _path: IList<string>;

    constructor() {
        this._path = IList();
    }

    get value(): IList<string> {
        return this._path;
    }

    hasCurrentSet(path: IList<string>): boolean {
        return this._path.equals(path);
    }

    @action goTo(newPath: IList<string>) {
        this._path = newPath;
    }
}
