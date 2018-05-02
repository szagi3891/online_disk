//@flow

import * as React from 'react';
import { List as IList } from 'immutable';
import { observer } from 'mobx-react';
import glamorous from 'glamorous';
import { Store } from '../Store';
import rgba from 'hex-rgba';

const PathItemBox = glamorous.span({
    paddingRight: '5px'
});

const PathItemNoActive = glamorous(PathItemBox)({
    cursror: 'default'
});

const PathItemSpanClick = glamorous(PathItemBox)({
    cursor: 'pointer',
    color: rgba('#0000ff', 50),
    ':hover': {
        opacity: '0.5'
    }
});

type PropsType = {|
    store: Store,
    caption: string,
    goToPath: IList<string> | null
|};

@observer
export class PathItem extends React.Component<PropsType> {
    render(): React.Node {
        const { goToPath, caption } = this.props;

        if (goToPath !== null) {
            return (
                <React.Fragment>
                    <PathItemSpanClick onClick={this._onClick}>
                        { caption }
                    </PathItemSpanClick>
                    <PathItemBox>&gt;</PathItemBox>
                </React.Fragment>
            );
        }

        return (
            <PathItemNoActive>
                { caption }
            </PathItemNoActive>
        );
    }

    _onClick = () => {
        const { store, goToPath } = this.props;
        if (goToPath !== null) {
            store.path.goTo(goToPath);
        }
    }
}
