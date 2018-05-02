//@flow

import * as React from 'react';
import { observer } from 'mobx-react';
import { Store } from '../Store';
import { PathItem } from './PathItem';

type PropsType = {|
    store: Store
|};

@observer
export class Path extends React.Component<PropsType> {
    render(): React.Node {
        const { store } = this.props;
        const fullPath = store.path.value;

        const out = [];

        for (let amountItem = 1; amountItem <= fullPath.size; amountItem++) {
            const key = `${fullPath.join('/')}--${amountItem}`;
            out.push(
                <PathItem
                    key={key}
                    store={store}
                    fullPath={fullPath}
                    amountItem={amountItem}
                />
            );
        }

        return (
            <div>
                path: { out }
            </div>
        );
    }
}
