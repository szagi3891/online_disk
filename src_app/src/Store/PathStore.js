//@flow
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

    @action goTo(newPath: IList<string>) {
        this._path = newPath;
    }
}
