//@flow

import * as React from 'react';
import { observer } from 'mobx-react';

import { Store } from '../Store';
import { DirAdd } from './DirAdd';
import { DirList } from './DirList';
import { Path } from './Path';

type PropsType = {|
    store: Store,
|};

@observer
export class App extends React.Component<PropsType> {
    render(): React.Node {
        const { store } = this.props;
        return (
            <React.Fragment>
                { this._renderHead() }
                <hr/>
                <Path store={store} />
                <hr/>
                { this._renderAddDir() }
                <hr/>
                { this._renderAddEmptyTextFile() }
                <hr/>
                { this._renderDirList() }
            </React.Fragment>
        );
    }

    _renderAddDir() {
        const { store } = this.props;
        const pathDir = store.root.currentPathNodes.path;
        const last = pathDir.last();

        if (last) {
            return (
                <DirAdd store={store} dirItem={last} />
            );
        }

        return null;
    }

    _renderAddEmptyTextFile() {
        return (
            <div>Dodaj pusty plik tekstowy</div>
        );
    }

    _renderDirList() {
        const { store } = this.props;
        const pathDir = store.root.currentPathNodes.path;
        const last = pathDir.last();

        if (last) {
            return (
                <DirList store={store} dirItem={last} />
            );
        }

        return (
            <div>Ładowanie listy ...</div>
        );
    }

    _renderHead(): React.Node {
        const { store } = this.props;
        const head = store.head;
        if (head === null) {
            return '---';
        }

        return (
            <div>{ head.head } - { head.counter }</div>
        );
    }
}
