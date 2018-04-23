//@flow

import * as React from 'react';
import { observable } from 'mobx';
import { observer } from 'mobx-react';
import { Store } from '../Store';

type PropsType = {|
    store: Store,
    hash: string
|};

type StateType = {|
    store: Store,
|};

@observer
export class DirList extends React.Component<PropsType, StateType> {
    state = {
        store: this.props.store
    }

    static getDerivedStateFromProps(nextProps: PropsType, prevState: StateType) {
        prevState.store.getDir(nextProps.hash);
    }

    render(): React.Node {
        return (
            <div>
                list z hashem
            </div>
        );
    }
}
