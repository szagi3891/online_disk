//@flow

import * as React from 'react';
import { observable } from 'mobx';
import { observer } from 'mobx-react';

import { Store } from '../Store';
import { DirAdd } from './DirAdd';
import { DirList } from './DirList';

const store = new Store();

type PropsType = {|
|};

@observer
export class App extends React.Component<PropsType> {
    constructor(props: PropsType) {
        super(props);
        store.getHead();
    }

    render(): React.Node {
        return (
            <React.Fragment>
                <div>
                    <button onClick={this._getHead}>Pobierz heada</button>
                </div>
                <div>
                    { this._renderHead() }
                </div>
                <DirAdd store={store} />
                { this._renderDirList() }
            </React.Fragment>
        );
    }

    _renderDirList() {
        const currentHead = store.head.head;
        if (currentHead !== null) {
            return (
                <DirList store={store} hash={currentHead} />
            );
        } else {
            return (
                <div>Ładowanie listy ...</div>
            );
        }
    }

    _getHead = () => {
        store.getHead();
    }

    _renderHead(): React.Node {
        const head = store.head;
        if (head === null) {
            return '---';
        }

        return (
            <div>
                <div>{ head.head }</div>
                <div>{ head.counter }</div>
            </div>
        );
    }
}
