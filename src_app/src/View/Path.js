//@flow

import * as React from 'react';
import { observer } from 'mobx-react';
import glamorous from 'glamorous';
import { Store } from '../Store';
import { PathItem } from './PathItem';

const Main = glamorous.div({
    display: 'flex'
});

type PropsType = {|
    store: Store
|};

@observer
export class Path extends React.Component<PropsType> {
    render(): React.Node {
        const { store } = this.props;
        const fullPath = store.path.value;

        const out = [];

        for (let amountItem = 0; amountItem <= fullPath.size; amountItem++) {
            const key = `${fullPath.join('/')}--${amountItem}`;

            const itemUrl = fullPath.slice(0, amountItem);
            const goToPath = amountItem < fullPath.size ? itemUrl : null;
            const caption = itemUrl.last();

            out.push(
                <PathItem
                    key={key}
                    caption={typeof caption === 'string' ? caption : 'ROOT'}
                    store={store}
                    goToPath={goToPath}
                />
            );
        }

        return (
            <Main>
                { out }
            </Main>
        );
    }
}
