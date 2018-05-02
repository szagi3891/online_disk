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
