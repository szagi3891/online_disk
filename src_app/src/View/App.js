//@flow

import * as React from 'react';
import { observable } from 'mobx';
import { observer } from 'mobx-react';

import { Store } from '../Store';
import { DirAdd } from './DirAdd';
import { DirList } from './DirList';
import { Path } from './Path';

const store = new Store();

type PropsType = {|
|};

@observer
export class App extends React.Component<PropsType> {
    render(): React.Node {
        const root = store.root.root;

        return (
            <React.Fragment>
                <div>
                    { this._renderHead() }
                </div>
                <hr/>
                <Path store={store} />
                <hr/>
                { this._renderAdd() }
                <hr/>
                { this._renderDirList() }
            </React.Fragment>
        );
    }

    _renderAdd() {
        const pathDir = store.root.currentPathNodes.path;
        const last = pathDir.last();

        if (last) {
            return (
                <DirAdd store={store} dirItem={last} />
            );
        }

        return null;
    }

    _renderDirList() {
        const pathDir = store.root.currentPathNodes.path;
        const last = pathDir.last();

        if (last) {
            return (
                <DirList store={store} dirItem={last} />
            );
        } else {
            return (
                <div>Ładowanie listy ...</div>
            );
        }
    }

    _renderHead(): React.Node {
        const head = store.head;
        if (head === null) {
            return '---';
        }

        return (
            <div>{ head.head } - { head.counter }</div>
        );
    }
}
