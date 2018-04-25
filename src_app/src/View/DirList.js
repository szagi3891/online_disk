//@flow

import * as React from 'react';
import { observable } from 'mobx';
import { observer } from 'mobx-react';
import { Store } from '../Store';

type PropsType = {|
    store: Store,
    hash: string
|};

@observer
export class DirList extends React.Component<PropsType> {
    render(): React.Node {
        const { store, hash } = this.props;

        const list = store.dir.getDir(hash);

        if (!list) {
            return (
                <div>
                    <i>Loading...</i>
                </div>
            );
        }

        return (
            <div>
                list z hashem
            </div>
        );
    }
}
