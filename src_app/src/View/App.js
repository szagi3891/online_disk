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
                { root !== null ? <DirAdd store={store} dirItem={root} /> : null }
                <hr/>
                { this._renderDirList() }
            </React.Fragment>
        );
    }

    _renderDirList() {
        const root = store.root.root;

        if (root !== null) {
            return (
                <DirList store={store} dirItem={root} />
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
